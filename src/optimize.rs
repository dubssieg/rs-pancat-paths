use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::process::exit;

pub fn relocate_ids(file_path: &str, output_file_node_ids: &str) -> io::Result<()> {
    /*
    This function reads a GFA file and prints each line with modified IDs, and outputs to other file the node mapping
     */
    let file: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut node_mappings: HashMap<u32, u32> = HashMap::new();
    let output_file = File::create(output_file_node_ids).expect("unable to create file");
    let mut file_writer = BufWriter::new(output_file);
    let mut line: String = String::new();
    let mut e_lines: Vec<String> = Vec::new();
    let mut new_id: u32 = 1;

    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'S' {
                // Rename nodes given a linear contiguous attribution
                let node_name: u32 = columns[1].parse::<u32>().unwrap();
                write!(file_writer, "{}\t{}\n", node_name, new_id)
                    .expect("Unable to write in output file.");
                node_mappings.insert(node_name, new_id);
                print!("{}\t{}\t{}", columns[0], new_id, columns[2]);
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
                println!("{}\t{}\t{}", columns[0], columns[1], chain);
            } else if first_char == 'W' {
                eprintln!("W-lines not implemented yet");
                exit(1);
            } else {
                print!("{}", line)
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
            print!(
                "{}\t{}\t{}\t{}\t{}\t{}",
                cols[0], mapped_id_from, cols[2], mapped_id_to, cols[4], cols[5]
            )
        }
    }

    Ok(())
}
