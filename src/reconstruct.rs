use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_pathlist_file(file_path: &str) -> io::Result<Vec<String>> {
    let file: File = File::open(file_path)?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut path_names: Vec<String> = Vec::new();

    for line in reader.lines() {
        path_names.push(line?);
    }

    Ok(path_names)
}

pub fn reconstruct_paths(file_path: &str, path_names: Vec<&str>) -> io::Result<()> {
    /*
    Reconstruct paths in the graph as their fasta sequences
    For each path in the graph, it will print to standard output the fasta sequence of the path
    The fasta sequence is made of the concatenation, in the right orientation, of each node described in the path
     */
    let file: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut seq_sequences: HashMap<String, String> = HashMap::new();
    let mut line: String = String::new();

    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'S' {
                // In the case of an S-line, we store the node name and the sequence
                let node_name: String = String::from(columns[1]);
                let sequence: String = String::from(columns[2].trim());
                seq_sequences.insert(node_name, sequence);
            }
            if first_char == 'P' {
                let path_name: String = String::from(columns[1]);
                if path_names.is_empty() || path_names.contains(&path_name.as_str()) {
                    let mut path_sequence: String = String::new();
                    let node_list: Vec<String> = columns[2]
                        .trim()
                        .split(',')
                        .map(|s| s.to_string())
                        .collect();
                    for node_desc in node_list {
                        let (node, orientation) = node_desc.split_at(node_desc.len() - 1);
                        let sequence: &String = seq_sequences.get(node).unwrap();
                        if orientation == "+" {
                            path_sequence.push_str(sequence);
                        } else {
                            path_sequence.push_str(&reverse_complement(sequence));
                        }
                    }
                    println!(">{}", path_name);
                    println!("{}", path_sequence);
                }
            }
        }
        line.clear(); // Clear the line buffer for the next read
    }
    Ok(())
}

fn reverse_complement(sequence: &String) -> String {
    let mut reverse_complement: String = String::new();
    for base in sequence.chars().rev() {
        match base {
            'A' => reverse_complement.push('T'),
            'C' => reverse_complement.push('G'),
            'G' => reverse_complement.push('C'),
            'T' => reverse_complement.push('A'),
            _ => reverse_complement.push(base),
        }
    }
    return reverse_complement;
}
