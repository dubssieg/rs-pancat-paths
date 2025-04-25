use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn find_spurious_breakpoints(file_path: &str) -> io::Result<()> {
    /*
    Given a file path, this function reads the GFA file and returns a HashMap:
    - spurious_nodes: a vector of spurious node IDs as values
    */
    let file: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    // STEP 1 : creating the HashMap
    // We use i32 as it is more than enough to store identifiers
    // We store node IDs as signed integers, with the sign giving the reading direction
    let mut seq_successors: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut seq_predecessors: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut line: String = String::new();
    // We parse the lines of the file
    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'L' {
                // In the case of an L-line, we store the predecessor and successor nodes
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
                // We update the Hashmaps with the nodes
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
                if seq_predecessors.contains_key(&to_node) {
                    if seq_predecessors
                        .get_mut(&to_node)
                        .unwrap()
                        .contains(&from_node)
                    {
                        continue;
                    } else {
                        seq_predecessors
                            .get_mut(&to_node)
                            .unwrap()
                            .push(from_node.clone());
                    }
                } else {
                    seq_predecessors.insert(to_node.clone(), vec![from_node.clone()]);
                }
            }
            line.clear(); // Clear the line buffer for the next read
        }
    }
    let mut node_pairs: HashSet<Vec<u32>> = HashSet::new();
    // STEP 2 : two-pass filter
    for (node, successors) in seq_successors.iter() {
        // checking if x has only one successor y accessed by the same sign
        if successors.len() == 1 && node.signum() == successors[0].signum() {
            // then checking if either reverse y has no reverse successors or have reverse x as a sole reverse successor
            if !seq_successors.contains_key(&-successors[0])
                || seq_successors
                    .get(&-successors[0])
                    .map_or(false, |v| v.len() == 1 && v[0] == -node)
            {
                if seq_predecessors
                    .get(&successors[0])
                    .map_or(false, |v| v.len() == 1)
                {
                    node_pairs.insert(vec![node.abs() as u32, successors[0].abs() as u32]);
                }
            }
        }
    }
    print_hashset(node_pairs);
    // STEP 3: We need to merge chains of spurious

    Ok(())
}

fn print_hashmap(hashmap: HashMap<i32, Vec<i32>>) {
    for (node, labels) in hashmap.iter() {
        print!("{}: ", node);
        for label in labels.iter() {
            print!("{}, ", label);
        }
        println!();
    }
}

fn print_hashset(hashmap: HashSet<Vec<u32>>) {
    for (node_pair) in hashmap.iter() {
        print!("{:?} ", node_pair);
    }
    println!();
}

fn clear_spurious_breakpoints(file_path: &str) -> io::Result<()> {
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
