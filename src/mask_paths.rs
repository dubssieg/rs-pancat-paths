use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom};

fn index_gfa(
    file_path: &str,
    hard_match: bool,
) -> io::Result<(HashMap<String, u64>, HashMap<String, char>)> {
    /*
    Given a file path, this function reads the GFA file and returns two HashMaps:
    - path_positions: a HashMap with the path names as keys and the offset of the path description as values
    - path_types: a HashMap with the path names as keys and the letter code as values
    */
    let file: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(file);

    let mut path_positions: HashMap<String, u64> = HashMap::new();
    let mut path_types: HashMap<String, char> = HashMap::new();

    let mut line: String = String::new();
    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'W' {
                path_types.insert(columns[1].to_string(), 'W');
                // In the case of a W-line, we store the path name and the offset of the path description
                // When processing paths, we can match paths in the path_positions HashMap
                // Then start reading the file from there and go with a buffer to read node by node the path
                let path_name: String = if hard_match {
                    columns[1].to_string() + "#" + columns[2] + "#" + columns[3]
                } else {
                    // Capitalize path name and remove trailing '#0'
                    (columns[1].to_string() + "#" + columns[2] + "#" + columns[3])
                        .to_ascii_uppercase()
                        .trim_end_matches("#0")
                        .to_string()
                };
                let offset: u64 = reader.seek(io::SeekFrom::Current(0))?
                    - (line.len() as u64
                        - columns[0].len() as u64
                        - columns[1].len() as u64
                        - columns[2].len() as u64
                        - columns[3].len() as u64
                        - columns[4].len() as u64
                        - columns[5].len() as u64
                        - 6);
                path_positions.insert(path_name.clone(), offset + 1);
            } else if first_char == 'P' {
                path_types.insert(columns[1].to_string(), 'P');
                // In the case of a P-line, we store the path name and the offset of the path description
                // When processing paths, we can match paths in the path_positions HashMap
                // Then start reading the file from there and go with a buffer to read node by node the path
                let path_name: String = if hard_match {
                    String::from(columns[1])
                } else {
                    // Capitalize path name and remove trailing '#0'
                    String::from(columns[1])
                        .to_ascii_uppercase()
                        .trim_end_matches("#0")
                        .to_string()
                };
                let offset: u64 = reader.seek(io::SeekFrom::Current(0))?
                    - (line.len() as u64 - columns[0].len() as u64 - columns[1].len() as u64 - 2);
                path_positions.insert(path_name.clone(), offset);
            }
        }
        line.clear(); // Clear the line buffer for the next read
    }

    Ok((path_positions, path_types))
}

pub fn mask_paths(file_path: &str, selected_paths: Vec<&str>) -> io::Result<()> {
    /*
     */
    eprintln!("Removing paths {:?} from {}", selected_paths, file_path);

    // STEP 1: extract path information from the graph
    let (mut path_positions, path_types) = match index_gfa(file_path, true) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error indexing GFA: {}", e);
            return Err(e);
        }
    };
    // STEP 2: gather nodes present in selected paths
    let mut gfa: BufReader<File> = BufReader::new(File::open(file_path)?);

    let mut node_arena: HashSet<u32> = HashSet::new();
    // We read each path
    for path_name in selected_paths {
        // Reading the file
        gfa.seek(SeekFrom::Start(*path_positions.get(path_name).ok_or_else(
            || {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Path not found in graph: {}", path_name),
                )
            },
        )?))?;
        let mut buffer: [u8; 1] = [0; 1];
        // Reading path contents
        if path_types.get(path_name) == Some(&'P') {
            while let Ok(node) = read_next_p_node(&mut gfa, &mut buffer).parse::<u32>() {
                node_arena.insert(node);
            }
        } else if path_types.get(path_name) == Some(&'W') {
            while let Ok(node) = read_next_w_node(&mut gfa, &mut buffer).parse::<u32>() {
                node_arena.insert(node);
            }
        }
        // We remove the selected_paths from the offsets hashmaps
        path_positions.remove(path_name);
    }
    // STEP 3: remove from the HashSet nodes that are present in other paths
    for r_path_name in path_positions.keys() {
        // Reading the file
        gfa.seek(SeekFrom::Start(
            *path_positions.get(r_path_name).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Path not found in graph: {}", r_path_name),
                )
            })?,
        ))?;
        let mut buffer: [u8; 1] = [0; 1];
        // Reading path contents
        if path_types.get(r_path_name) == Some(&'P') {
            while let Ok(node) = read_next_p_node(&mut gfa, &mut buffer).parse::<u32>() {
                node_arena.remove(&node);
            }
        } else if path_types.get(r_path_name) == Some(&'W') {
            while let Ok(node) = read_next_w_node(&mut gfa, &mut buffer).parse::<u32>() {
                node_arena.remove(&node);
            }
        }
    }
    // STEP 4: filter graph structures
    let file: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut line: String = String::new();
    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'S' {
                if let Ok(node_id) = columns[1].parse::<u32>() {
                    if !node_arena.contains(&node_id) {
                        print!("{}", line)
                    }
                }
            } else if first_char == 'L' {
                if let (Ok(node_id_from), Ok(node_id_to)) =
                    (columns[1].parse::<u32>(), columns[3].parse::<u32>())
                {
                    if !node_arena.contains(&node_id_from) && !node_arena.contains(&node_id_to) {
                        print!("{}", line)
                    }
                }
            } else if first_char == 'W' {
                if path_positions
                    .contains_key(&(columns[1].to_string() + "#" + columns[2] + "#" + columns[3]))
                {
                    print!("{}", line)
                }
            } else if first_char == 'P' {
                if path_positions.contains_key(columns[1]) {
                    print!("{}", line)
                }
            } else {
                print!("{}", line)
            }
        }
        line.clear(); // Clear the line buffer for the next read
    }

    Ok(())
}

fn read_next_p_node(file: &mut BufReader<File>, buffer: &mut [u8; 1]) -> String {
    /*
     * Read the next node in the file, until a comma is found
     * file: the file to read
     * buffer: a buffer to read the file
     */
    let mut node: String = String::new();
    while file.read(buffer).unwrap() > 0 {
        if buffer[0] == b'+' || buffer[0] == b'-' {
            break;
        }
        if buffer[0] != b',' {
            node.push(buffer[0] as char);
        }
    }
    node
}

fn read_next_w_node(file: &mut BufReader<File>, buffer: &mut [u8; 1]) -> String {
    /*
     * Read the next node in the file, until a > or < is found
     * file: the file to read
     * buffer: a buffer to read the file
     */
    let mut node: String = String::new();
    while file.read(buffer).unwrap() > 0 {
        if buffer[0] == b'>' || buffer[0] == b'<' || buffer[0] == b'\t' {
            break;
        } else {
            node.push(buffer[0] as char);
        }
    }
    node
}
