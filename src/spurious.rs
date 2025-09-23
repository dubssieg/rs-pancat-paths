use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn prune_spurious_breakpoints(file_path: &str) -> io::Result<()> {
    /*
    Given a file path, this function reads the GFA file and returns a HashMap:
    - spurious_nodes: a vector of spurious node IDs as values
    */
    let file: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    // STEP 1 : creating the HashMap
    eprintln!("STEP 1 --- parsing GFA file");
    // We use i32 as it is more than enough to store identifiers
    // We store node IDs as signed integers, with the sign giving the reading direction
    let mut seq_successors: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut seq_predecessors: HashMap<i32, Vec<i32>> = HashMap::new();
    // mapping storage (for node IDs)
    let mut mapping: HashMap<u32, u32> = HashMap::new();
    let mut nodes_sequences: HashMap<u32, String> = HashMap::new();

    let mut line: String = String::new();
    // We parse the lines of the file
    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'S' {
                let node_id: u32 = columns[1].parse::<u32>().unwrap();
                mapping.insert(node_id, node_id);
                let mut sequence: String = columns[2].parse::<String>().unwrap();
                if sequence.ends_with('\n') {
                    sequence.pop();
                }
                nodes_sequences.insert(node_id, sequence);
            }
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
    eprintln!("STEP 2 --- two-pass filter");
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
    // STEP 3: We need to merge chains of spurious
    eprintln!("STEP 3 --- popping spurious breakpoints");
    let mut pop_vect: Vec<u32> = Vec::new();
    // easiest way is to retain a Hashmap of node IDs, change them as we go through merging
    for pair in node_pairs.iter() {
        // update sequence of the node and delete other node
        // /!\ takes currently into account only forward direction
        let new_sequence:String = format!("{:?}{:?}",nodes_sequences.get(&pair[0]),nodes_sequences.get(&pair[1]));
        nodes_sequences.insert(pair[0], new_sequence);
        pop_vect.push(pair[1]);
        nodes_sequences.remove(&pair[1]);
        // update mapping
        mapping.insert(pair[1], pair[0]);
    }

    // STEP 4: write output file
    eprintln!("STEP 4 --- write to output GFA");
    // we filter nodes that no longer exists
    let file: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(file);

    let mut line: String = String::new();
    // We parse the lines of the file
    while reader.read_line(&mut line)? > 0 {
        if line.ends_with('\n') {
            line.pop();
        }
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'S' {
                let node_id: u32 = columns[1].parse::<u32>().unwrap();
                if mapping.get(&node_id) == Some(&node_id) {
                    println!("S\t{}\t{}",node_id,nodes_sequences.get(&node_id).unwrap())
                }
            }
            else if first_char == 'L' {
                let in_node: u32 = resolve_dep(&mapping,columns[1].parse::<u32>().unwrap());
                let out_node: u32 = resolve_dep(&mapping,columns[3].parse::<u32>().unwrap());
                if in_node != out_node {
                    println!("L\t{}\t{}\t{}\t{}\t{}",mapping.get(&in_node).unwrap(),columns[2],mapping.get(&out_node).unwrap(),columns[4],columns[5])
                }
            }
            else if first_char == 'P' {
                let mut node_list: Vec<String> = columns[2]
                    .trim()
                    .split(',')
                    .map(|s| s.to_string())
                    .collect();
                node_list.retain(|s| mapping.get(&(s[0..s.len() - 1].parse::<u32>().unwrap())) == Some(&(s[0..s.len() - 1].parse::<u32>().unwrap())));
                println!("P\t{}\t{}\t*",columns[1],node_list.join(","));
            }
            else if first_char == 'W' {
                let mut node_list: Vec<String> = columns[6][1..]
                    .trim()
                    .split_inclusive(&['>', '<'][..])
                    .map(|s| s.to_string())
                    .collect();
                // we add char on first item
                node_list[0] = String::from(columns[2].chars().nth(0).unwrap()) + &node_list[0];
                node_list.retain(|s| mapping.get(&(s[1..].parse::<u32>().unwrap())) == Some(&(s[1..].parse::<u32>().unwrap())));
                println!("W\t{}\t{}\t{}\t{}\t{}\t{}",columns[1],columns[2],columns[3],columns[4],columns[5],node_list.join(""));
            }
            else {
                println!("{}",line);
            }
        line.clear(); // Clear the line buffer for the next read
        }
    }
    Ok(())
}

fn resolve_dep(mapping: &HashMap<u32,u32>, start:u32) -> u32 {
    /*
    Resolves a chain of dependancies until a existing node is found in mapping
     */
    let mut id:u32 = start;
    while Some(&id) != mapping.get(&id) {
        eprintln!("{} != {} ? {}", id,mapping.get(&id).unwrap(),Some(&id) != mapping.get(&id));
        id = *mapping.get(&id).unwrap();
    }
    return id;
}