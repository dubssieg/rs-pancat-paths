use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn index_gfa(file_path: &str) -> io::Result<()> {
    /*
    This function reads a GFA file and prints the length of each path
     */
    let file: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut seq_lengths: HashMap<String, u64> = HashMap::new();
    let mut line: String = String::new();

    println!(
        "# {}\t{}\t{}\t{}",
        "PathName", "Length", "ForwardLength", "ReverseLength"
    );
    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'S' {
                // In the case of an S-line, we store the node name and the sequence length
                let node_name: String = String::from(columns[1]);
                let sequence_length: u64 = columns[2].trim().len() as u64;
                seq_lengths.insert(node_name, sequence_length);
            }
            if first_char == 'P' {
                let mut path_length: u64 = 0;
                let mut path_length_forward: u64 = 0;
                let mut path_length_reverse: u64 = 0;
                let path_name: String = String::from(columns[1]);

                let node_list: Vec<String> = columns[2]
                    .trim()
                    .split(',')
                    .map(|s| s.to_string())
                    .collect();
                for node_desc in node_list {
                    let (node, orientation) = node_desc.split_at(node_desc.len() - 1);
                    let sequence_length: &u64 = seq_lengths.get(node).unwrap();
                    path_length += sequence_length;
                    if orientation == "+" {
                        path_length_forward += sequence_length;
                    } else {
                        path_length_reverse += sequence_length;
                    }
                }
                println!(
                    "{}\t{}\t{}\t{}",
                    path_name, path_length, path_length_forward, path_length_reverse
                );
            }
        }
        line.clear(); // Clear the line buffer for the next read
    }

    Ok(())
}

pub fn rename_paths(file_path: &str, rename_file: &str) -> io::Result<()> {
    /*
    This function reads a GFA file and a TSV file with two columns: old_name and new_name
    It replaces the names of the paths in the GFA file with the new names
     */
    let file: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut line: String = String::new();

    let rename_file: File = File::open(rename_file)?;
    let mut rename_reader: BufReader<File> = BufReader::new(rename_file);
    let mut rename_line: String = String::new();
    let mut rename_map: HashMap<String, String> = HashMap::new();

    // Read the rename file and store the old and new names in a hashmap
    while rename_reader.read_line(&mut rename_line)? > 0 {
        let columns: Vec<&str> = rename_line.split('\t').collect();
        let old_name: String = String::from(columns[0]);
        let new_name: String = String::from(columns[1].strip_suffix("\n").unwrap());
        rename_map.insert(old_name, new_name);
        rename_line.clear();
    }

    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'P' {
                println!(
                    "{}\t{}\t{}",
                    columns[0],
                    rename_map.get(columns[1]).unwrap(),
                    columns[2..].join("\t")
                );
            } else {
                println!("{}", line);
            }
        }
        line.clear(); // Clear the line buffer for the next read
    }

    Ok(())
}

pub fn offset_gfa(file_path: &str) -> io::Result<()> {
    /*
    This function reads a GFA file and prints the length of each path
     */
    let file: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut seq_lengths: HashMap<String, u64> = HashMap::new();
    let mut line: String = String::new();

    println!(
        "# {}\t{}\t{}\t{}\t{}\t{}",
        "NodeName", "Path", "StartPos", "EndPos", "Length", "Orientation"
    );
    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'S' {
                // In the case of an S-line, we store the node name and the sequence length
                let node_name: String = String::from(columns[1]);
                let sequence_length: u64 = columns[2].trim().len() as u64;
                seq_lengths.insert(node_name, sequence_length);
            }
            if first_char == 'P' {
                let mut path_length: u64 = 0;
                let path_name: String = String::from(columns[1]);

                let node_list: Vec<String> = columns[2]
                    .trim()
                    .split(',')
                    .map(|s| s.to_string())
                    .collect();
                for node_desc in node_list {
                    let (node, orientation) = node_desc.split_at(node_desc.len() - 1);
                    let sequence_length: &u64 = seq_lengths.get(node).unwrap();
                    path_length += sequence_length;
                    println!(
                        "{}\t{}\t{}\t{}\t{}\t{}",
                        node,
                        path_name,
                        path_length,
                        path_length + sequence_length,
                        sequence_length,
                        orientation
                    );
                }
            }
        }
        line.clear(); // Clear the line buffer for the next read
    }

    Ok(())
}
