use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
/*
struct Interval {
    start: i32,
    end: i32,
}

fn add_interval(interval: Interval, intervals: &mut Vec<Interval>) {
    /*
    This function adds an interval to a list of intervals
     */
    let mut new_intervals: Vec<Interval> = Vec::new();
    let mut added: bool = false;
    for i in intervals.iter() {
        if i.end < interval.start {
            new_intervals.push(i.clone());
        } else if i.start > interval.end {
            if !added {
                new_intervals.push(interval);
                added = true;
            }
            new_intervals.push(i.clone());
        } else {
            let new_interval: Interval = Interval {
                start: i.start.min(interval.start),
                end: i.end.max(interval.end),
            };
            new_intervals.push(new_interval);
            added = true;
        }
    }
    if !added {
        new_intervals.push(interval);
    }
    *intervals = new_intervals;
}

*/

pub fn shared_nodes(
    file_path: &str,
    include: &Vec<String>,
    exclude: &Vec<String>,
    sensitivity: f64,
) -> io::Result<()> {
    /*
    This function reads a GFA file ad two lists of paths
    The list of paths to include designates the paths that must be shared by the nodes, up to a given sensitivity ratio
    The list of paths to exclude designates the paths that must not be shared by the nodes, up to 1 - a given sensitivity ratio
    It prints the paths that share the given intervals
     */

    // We create for each node a vector of boolean values, one for each path in the include and the exclude lists
    // If the node is in the path, the value is true, otherwise it is false
    // We then compare the vectors to see if the node is in the paths we want to include and not in the paths we want to exclude up to the sensitivity ratio

    let file: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut segments_vectors: HashMap<String, Vec<bool>> = HashMap::new();
    let mut line: String = String::new();
    let paths: Vec<String> = include
        .iter()
        .chain(exclude.iter())
        .map(|s| s.to_string())
        .collect();

    println!(
        "# NodeName\tStatus\tIncludeRatio\tExcludeRatio\t{:?}",
        paths
    );
    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'S' {
                // In the case of an S-line, we store the node name and the sequence length
                let node_name: String = String::from(columns[1]);
                let boolean_vector: Vec<bool> = vec![false; paths.len()];
                segments_vectors.insert(node_name, boolean_vector);
            }
            if first_char == 'P' {
                let path_name: String = String::from(columns[1]);

                let node_list: Vec<String> = columns[2]
                    .trim()
                    .split(',')
                    .map(|s| s.to_string())
                    .collect();
                for node_desc in node_list {
                    let (node, _orientation) = node_desc.split_at(node_desc.len() - 1);
                    // Edit the correct boolean position in the corresponding vector
                    let boolean_vector: &mut Vec<bool> = segments_vectors.get_mut(node).unwrap();
                    let mut i: usize = 0;
                    for path in paths.iter() {
                        if path == &path_name {
                            boolean_vector[i] = true;
                        }
                        i += 1;
                    }
                }
            }
        }
        line.clear(); // Clear the line buffer for the next read
    }
    for node in segments_vectors.keys() {
        let boolean_vector: Vec<i32> = segments_vectors
            .get(node)
            .unwrap()
            .iter()
            .map(|&b| b as i32)
            .collect();
        let mut include_count: i32 = 0;
        let mut exclude_count: i32 = 0;
        for i in 0..paths.len() {
            if include.contains(&paths[i]) {
                include_count += boolean_vector[i];
            }
            if exclude.contains(&paths[i]) {
                exclude_count += boolean_vector[i];
            }
        }
        let include_ratio: f64 = include_count as f64 / include.len() as f64;
        let exclude_ratio: f64 = exclude_count as f64 / exclude.len() as f64;
        let shared: bool = (include_ratio.is_nan() || include_ratio >= sensitivity)
            && (exclude_ratio.is_nan() || exclude_ratio <= 1.0 - sensitivity);

        println!(
            "{}\t{}\t{}\t{}\t{:?}",
            node, shared as i32, include_ratio, exclude_ratio, boolean_vector
        );
    }

    Ok(())
}
