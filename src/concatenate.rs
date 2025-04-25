use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::exit;

pub fn concat_graphs(file_path: &str, graph_to_append: &str) -> io::Result<()> {
    /*
    This function reads a GFA file and prints each line with modified IDs, and outputs to other file the node mapping
     */
    let file_a: File = File::open(file_path)?;
    let mut reader_a: BufReader<File> = BufReader::new(file_a);
    let mut line: String = String::new();
    let mut max_node_id: u32 = 0;

    // We seek for the maximum node id in the first graph
    while reader_a.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'S' {
                // Rename nodes given a linear contiguous attribution
                let node_name: u32 = columns[1].parse::<u32>().unwrap();
                max_node_id = max(node_name, max_node_id);
            }
            print!("{}", line)
        }
        line.clear(); // Clear the line buffer for the next read
    }

    // next, we iterate on the second file
    let file_b: File = File::open(graph_to_append)?;
    let mut reader_b: BufReader<File> = BufReader::new(file_b);
    let mut node_mappings: HashMap<u32, u32> = HashMap::new();
    let mut line: String = String::new();
    let mut e_lines: Vec<String> = Vec::new();
    let mut new_id: u32 = max_node_id + 1;

    while reader_b.read_line(&mut line)? > 0 {
        if line.ends_with('\n') || line.ends_with('\r') {
            line.pop();
        }
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'S' {
                // Rename nodes given a linear contiguous attribution
                let node_name: u32 = columns[1].parse::<u32>().unwrap();
                node_mappings.insert(node_name, new_id);
                println!("{}\t{}\t{}", columns[0], new_id, columns[2..].join("\t"));
                new_id += 1;
            } else if first_char == 'L' {
                // Store line for later
                e_lines.push(line.clone());
            } else if first_char == 'P' {
                // Rename all nodes given the map
                let mut chain: String = String::new();
                let node_list: Vec<String> = columns[2]
                    .trim()
                    .split(',')
                    .map(|s| s.to_string())
                    .collect();
                for node_desc in node_list {
                    let (node, orientation) = node_desc.split_at(node_desc.len() - 1);
                    if let Some(&mapped_id) = node_mappings.get(&node.parse::<u32>().unwrap()) {
                        chain += &format!("{}{},", mapped_id, orientation);
                    } else {
                        eprintln!("Node {} not found in mappings", node);
                        exit(1);
                    }
                }
                chain.truncate(chain.len() - 1);
                println!("{}\t{}\t{}\t{}", columns[0], columns[1], chain, columns[3]);
            } else if first_char == 'W' {
                eprintln!("W-lines not implemented yet");
                exit(1);
            }
        }
        line.clear(); // Clear the line buffer for the next read
                      // Treat L-lines
    }
    for edge in e_lines {
        let cols: Vec<&str> = edge.split('\t').collect();
        if let (Some(&mapped_id_from), Some(&mapped_id_to)) = (
            node_mappings.get(&cols[1].parse::<u32>().unwrap()),
            node_mappings.get(&cols[3].parse::<u32>().unwrap()),
        ) {
            println!(
                "{}\t{}\t{}\t{}\t{}\t{}",
                cols[0], mapped_id_from, cols[2], mapped_id_to, cols[4], cols[5]
            )
        }
    }

    Ok(())
}
