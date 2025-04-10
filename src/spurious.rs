use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn find_spurious_breakpoints(file_path: &str) -> io::Result<()> {
    /*
    Given a file path, this function reads the GFA file and returns a HashMap:
    - spurious_nodes: a vector of spurious node IDs as values
    */
    let file: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    // We use i32 as it is more than enough to store identifiers
    // We store node IDs as signed integers, with the sign giving the reading direction
    let mut seq_successors: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut line: String = String::new();
    // We parse the lines of the file
    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'E' {
                // In the case of an E-line, we store the predecessor and successor nodes
                let from_node: i32 = if columns[2] == "+" {
                    columns[1].parse::<i32>().unwrap()
                } else {
                    -columns[1].parse::<i32>().unwrap()
                };
                let to_node: i32 = if columns[4] == "+" {
                    columns[3].parse::<i32>().unwrap()
                } else {
                    -columns[3].parse::<i32>().unwrap()
                };

                if seq_successors.contains_key(&from_node) {
                    if seq_successors
                        .get_mut(&from_node)
                        .unwrap()
                        .contains(&to_node)
                    {
                        continue;
                    } else {
                        seq_successors
                            .get_mut(&from_node)
                            .unwrap()
                            .push(to_node.clone());
                    }
                } else {
                    seq_successors.insert(from_node.clone(), vec![to_node.clone()]);
                }
            }
            line.clear(); // Clear the line buffer for the next read
        }
    }
    // Then we perform a first filter 
    // We then search for nodes that have only one predecessor and we return them
    for (node, successors) in seq_successors.iter() {
        if successors.len() == 1 {
            println!("{}", node);
        }
    }
    Ok(())
}

pub fn clear_spurious_breakpoints(file_path: &str) -> io::Result<()> {
    /*
    This function reads a GFA file and prints the length of each path
     */
    let file: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut seq_lengths: HashMap<String, u64> = HashMap::new();
    let mut line: String = String::new();

    println!("The following paths are spurious breakpoints in the GFA file:");
    while reader.read_line(&mut line)? > 0 {
        // Process the line
        line.clear(); // Clear the line buffer for the next read
    }
    Ok(())
}
