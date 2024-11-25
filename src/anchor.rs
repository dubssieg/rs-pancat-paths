use indexmap::IndexMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn anchor_nodes(file_path: &str, max_rank: Option<i32>) -> io::Result<()> {
    /*
    This function reads a GFA file and for each node emits an annotation of the subset of paths that cross the node
     */
    let file: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut segment_replicates: IndexMap<String, i32> = IndexMap::new();
    let mut line: String = String::new();

    println!("# NodeName\tAnchorRank");
    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'S' {
                // In the case of an S-line, we store the node name and the sequence length
                let node_name: String = String::from(columns[1]);
                segment_replicates.insert(node_name, 0);
            }
            if first_char == 'P' {
                // Populates a vector with 0s for each node in the graph
                let mut boolean_vector: Vec<bool> = vec![false; segment_replicates.len()];

                let node_list: Vec<String> = columns[2]
                    .trim()
                    .split(',')
                    .map(|s| s.to_string())
                    .collect();
                for node_desc in node_list {
                    let (node, _orientation) = node_desc.split_at(node_desc.len() - 1);
                    boolean_vector[segment_replicates.get_full(node).unwrap().0] = true;
                }
                let keys: Vec<_> = segment_replicates.keys().cloned().collect();
                for node in keys {
                    let node_index: usize = segment_replicates.get_full(&node).unwrap().0;
                    if boolean_vector[node_index] {
                        *segment_replicates.get_mut(&node).unwrap() += 1;
                    }
                }
            }
        }
        line.clear(); // Clear the line buffer for the next read
    }
    if max_rank.is_some() {
        for node in segment_replicates.keys() {
            let obs_rank = *segment_replicates.values().max().unwrap();
            if segment_replicates[node] >= obs_rank - max_rank.unwrap() {
                println!("{}\t{}", node, segment_replicates[node]);
            }
        }
    } else {
        for node in segment_replicates.keys() {
            println!("{}\t{}", node, segment_replicates[node]);
        }
    }

    Ok(())
}
