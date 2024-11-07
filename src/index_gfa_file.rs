use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn index_gfa(file_path: &str) -> io::Result<()> {
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
