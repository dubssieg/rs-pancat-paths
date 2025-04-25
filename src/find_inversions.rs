use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn one_sized_bubbles(file_path: &str) -> io::Result<()> {
    /*
    Given a file path, this function reads the GFA file and returns a Vector:
    - inversion_nodes: a vector of inversion node IDs
    */
    let file: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut seq_successors: HashMap<String, Vec<String>> = HashMap::new();
    let mut seq_predecessors: HashMap<String, Vec<String>> = HashMap::new();

    let mut node_ids: Vec<String> = Vec::new();

    // Inversion nodes are defined by the following properties:
    // - They have only one predecessor and one successor
    // - They have edges that goes +/+ and +/- from predecessor to node and +/+ and -/+ from node to successor
    // - Or they have edges that goes -/- and -/+ from predecessor to node and -/- and +/- from node to successor

    // We will store node ids in a vector

    let mut line: String = String::new();
    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'S' {
                // In the case of an S-line, we store the node ID
                let node_id: String = String::from(columns[1]);
                node_ids.push(node_id);
            } else if first_char == 'E' {
                // In the case of an E-line, we store the predecessor and successor nodes
                let predecessor_name: String = String::from(columns[1]);
                let successor_name: String = String::from(columns[3]);
                let predecessor_orientation: String = String::from(columns[2]);
                let successor_orientation: String = String::from(columns[4]);
            }
            line.clear(); // Clear the line buffer for the next read
        }
    }
    // We then search for nodes that have only one predecessor and we return them
    for (node, successors) in seq_successors.iter() {
        if successors.len() == 1 {
            println!("{}", node);
        }
    }
    Ok(())
}
