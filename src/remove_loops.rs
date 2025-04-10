use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Seek};

pub fn remove_loops(file_path: &str, threshold: u8) {
    let paths: HashMap<String, HashMap<String, u8>> = read_paths(file_path, threshold).unwrap();
    let max_label: u64 = get_max_label(file_path);
    let mut available_label: u64 = max_label + 1;
    let mut aggregated_occurences: HashMap<String, u8> = aggregate_occurences(paths);

    // Allocate node labels in vectors for each node with multiple occurences.
    // Number of labels is equal to the aggregated_occurences count
    let mut node_labels: HashMap<String, Vec<u64>> = HashMap::new();
    for (node, count) in aggregated_occurences.iter() {
        let mut labels: Vec<u64> = Vec::new();
        for _ in 0..*count {
            labels.push(available_label);
            available_label += 1;
        }
        node_labels.insert(node.to_string(), labels);
    }
    //print_hashmap(node_labels);
    println!("Node count: {}", get_node_count(file_path));
    println!("Added labels: {}", available_label - max_label);
}

fn print_hashmap(hashmap: HashMap<String, Vec<u64>>) {
    for (node, labels) in hashmap.iter() {
        print!("{}: ", node);
        for label in labels.iter() {
            print!("{}, ", label);
        }
        println!();
    }
}

fn get_node_count(file_path: &str) -> u64 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut node_count: u64 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'S' {
                node_count += 1;
            }
        }
    }
    node_count
}

fn read_next_node<R: Read>(file: &mut BufReader<R>, buffer: &mut [u8; 1]) -> String {
    /*
     * Read the next node in the file, until a comma or a newline is found
     * Return the node name and a boolean indicating if the end of the line has been reached
     * file: the file to read
     * buffer: a buffer to read the file
     */
    let mut node = String::new();
    while file.read(buffer).unwrap() > 0 {
        if buffer[0] == b'+' || buffer[0] == b'-' {
            break;
        }
        if buffer[0] != b',' {
            node.push(buffer[0] as char);
        }
    }
    node
}

fn add_nodes_to_graph(file_path: &str, node_labels: HashMap<String, Vec<u64>>) {
    /*
    Adding a node to the graph implies to create a label, copy the sequence and add the node to the graph
    Reads each S line of the GFA.
    Print the node.
    If the node is a key of node_labels, create per occurence in the vector a new node
    Else, only print the line without any modification
     */
    let file = File::open(file_path).unwrap();
    // TODO
}

fn count_number_occurences_in_path<R: Read>(
    file: &mut BufReader<R>,
    buffer: &mut [u8; 1],
) -> HashMap<String, u8> {
    let mut number_nodes_in_path: u64 = 0;
    let mut occurences: HashMap<String, u8> = HashMap::new();
    loop {
        let node = read_next_node(file, buffer);
        if node.is_empty() {
            break;
        }
        let count = occurences.entry(node.to_string()).or_insert(0);
        *count += 1;
        number_nodes_in_path += 1;
    }
    println!("Number of nodes in path: {}", number_nodes_in_path);
    occurences
}

fn filter_occurences(occurences: HashMap<String, u8>, threshold: u8) -> HashMap<String, u8> {
    let mut filtered_occurences: HashMap<String, u8> = HashMap::new();
    for (node, count) in occurences.iter() {
        if *count > threshold {
            filtered_occurences.insert(node.to_string(), *count);
        }
    }
    filtered_occurences
}

fn aggregate_occurences(
    occurences_map: HashMap<String, HashMap<String, u8>>,
) -> HashMap<String, u8> {
    /*
    Across all paths, keeps the maximum number of occurences for each node
     */
    let mut aggregated_occurences: HashMap<String, u8> = HashMap::new();
    for occurences in occurences_map.values() {
        for (node, count) in occurences.iter() {
            let max_count = aggregated_occurences.entry(node.to_string()).or_insert(0);
            if count > max_count {
                *max_count = *count;
            }
        }
    }
    aggregated_occurences
}

fn read_paths(file_path: &str, threshold: u8) -> io::Result<HashMap<String, HashMap<String, u8>>> {
    /*
    Given a file path, returns the number of occurences of each node in each path
     */
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1];
    let mut paths: HashMap<String, HashMap<String, u8>> = HashMap::new();
    // Loop on paths found by the index_paths function
    let index: HashMap<String, u64> = index_paths(file_path)?;

    for (path_name, offset) in index {
        reader.seek(io::SeekFrom::Start(offset))?;
        let occurences: HashMap<String, u8> = filter_occurences(
            count_number_occurences_in_path(&mut reader, &mut buffer),
            threshold,
        );
        paths.insert(path_name, occurences);
    }

    Ok(paths)
}

fn get_max_label(file_path: &str) -> u64 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut max_label: u64 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'S' {
                let label: u64 = columns[1].parse().unwrap();
                if label > max_label {
                    max_label = label;
                }
            }
        }
    }
    max_label
}

fn index_paths(file_path: &str) -> io::Result<HashMap<String, u64>> {
    /*
    Given a file path, this function reads the GFA file and returns a HashMap:
    - path_positions: a HashMap with the path names as keys and the offset of the path description as values
    */
    let file: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(file);

    let mut path_positions: HashMap<String, u64> = HashMap::new();

    let mut line: String = String::new();
    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'W' {
                eprintln!("Error: W-lines support not implemented yet. Please convert your graph to GFA1.0");
                std::process::exit(1);
            }
            if first_char == 'P' {
                let path_name = String::from(columns[1]);
                let offset = reader.seek(io::SeekFrom::Current(0))?
                    - (line.len() as u64 - columns[0].len() as u64 - columns[1].len() as u64 - 2);
                path_positions.insert(path_name, offset);
            }
        }
        line.clear(); // Clear the line buffer for the next read
    }
    Ok(path_positions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_next_node() {
        let cursor = Cursor::new(b"1+,2+,3+");
        let mut buffer = [0; 1];
        let mut reader = BufReader::new(cursor);
        assert_eq!(read_next_node(&mut reader, &mut buffer), "1");
        assert_eq!(read_next_node(&mut reader, &mut buffer), "2");
        assert_eq!(read_next_node(&mut reader, &mut buffer), "3");
    }

    #[test]
    fn test_count_number_occurences_in_path() {
        let cursor = Cursor::new(b"1+,2+,3+,1+,2+");
        let mut buffer = [0; 1];
        let mut reader = BufReader::new(cursor);
        let occurences = count_number_occurences_in_path(&mut reader, &mut buffer);
        assert_eq!(occurences.get("1"), Some(&2));
        assert_eq!(occurences.get("2"), Some(&2));
        assert_eq!(occurences.get("3"), Some(&1));
    }

    fn test_filter_occurences() {
        let mut occurences: HashMap<String, u8> = HashMap::new();
        occurences.insert("1".to_string(), 2);
        occurences.insert("2".to_string(), 3);
        occurences.insert("3".to_string(), 1);
        let filtered_occurences = filter_occurences(occurences, 2);
        assert_eq!(filtered_occurences.get("1"), Some(&2));
        assert_eq!(filtered_occurences.get("2"), Some(&3));
        assert_eq!(filtered_occurences.get("3"), None);
    }
}
