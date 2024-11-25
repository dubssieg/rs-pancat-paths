use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek};

struct Tree {
    // A tree is a collection of nodes
    nodes: HashMap<i32, Node>,
}

#[derive(Clone)]
struct Path {
    // A path is a string name
    name: String,
    index: i32,
}

#[derive(Clone)]
struct Node {
    // A node is an identifier, a path, and a list of children (as identifiers)
    id: i32,
    path: Path,
    length: i32,
    offset: i32,
    children: Vec<i32>,
}

impl Tree {
    fn new() -> Tree {
        Tree {
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, id: i32, path: Path, length: i32, offset: i32) {
        self.nodes.insert(id, Node::new(id, path, length, offset));
    }

    fn add_child(&mut self, parent_id: i32, child_id: i32) {
        self.nodes
            .get_mut(&parent_id)
            .unwrap()
            .children
            .push(child_id);
    }

    fn get_node(&self, id: i32) -> &Node {
        self.nodes.get(&id).unwrap()
    }
}

impl Node {
    fn new(id: i32, path: Path, length: i32, offset: i32) -> Node {
        Node {
            id: id,
            path: path,
            length: length,
            offset: offset,
            children: Vec::new(),
        }
    }

    fn get_path(&self) -> &Path {
        &self.path
    }

    fn get_offset(&self) -> i32 {
        self.offset
    }
}

impl Path {
    fn new(name: String, index: i32) -> Path {
        Path {
            name: name,
            index: index,
        }
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_index(&self) -> i32 {
        self.index
    }
}

pub fn gfa_to_rgfa(file_path: &str, reference: &str) -> io::Result<()> {
    /*
    This function reads a GFA file and prints the rGFA version of the file
    rGFA is a subset of GFA, with only the S and L lines
    S lines are annotated with three supplementary fields: SN, SO, and SR
    * SN (string): Name of stable sequence from which the segment is derived
    * SO (integer): Offset on the stable sequence
    * SR (integer): Rank: 0 if on a linear reference genome, >0 otherwise.
    Due to the nature of the GFA format, all paths should be stored.

    Will only work on acyclic graphs.
    Problem is that the only tool that break cycles is odgi break.
    And odgi break drops paths when breaking cycles.
    If we use this on cyclic graphs, we will have inconsistent offsets (SO field).
     */
    let file: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut paths: HashMap<String, Vec<String>> = HashMap::new();
    let mut line: String = String::new();
    let mut sequence_lengths: HashMap<i32, i32> = HashMap::new();
    let mut path_index: i32 = 0;

    // We represent as a tree the paths that are given in the file
    // it is mandatory to derive the SO, SR, and SN fields
    let mut tree: Tree = Tree::new();

    // We need to read the file a first time to store node lists
    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'S' {
                let seq_name: i32 = columns[1].parse().unwrap();
                let seq_length: i32 = columns[2].len() as i32;
                sequence_lengths.insert(seq_name, seq_length);
            }
            if first_char == 'P' {
                let path_name: String = String::from(columns[1]);
                let node_list: Vec<String> = columns[2]
                    .trim()
                    .split(',')
                    .map(|s| s[..s.len() - 1].to_string())
                    .collect();
                paths.insert(path_name, node_list);
            }
        }
        line.clear(); // Clear the line buffer for the next read
    }
    // We fill the tree, reading path per path the graph
    let refpath: Path = Path::new(reference.to_string(), path_index);
    let mut offset: i32 = 0;

    // We start by the reference path
    let mut parent_node: Option<Node> = None;
    for node_name in paths.get(reference).unwrap() {
        let node_id: i32 = node_name.parse().unwrap();
        let node_length: i32 = sequence_lengths.get(&node_id).unwrap().clone();
        // We add the node if it is not already in the tree
        if !tree.nodes.contains_key(&node_id) {
            tree.add_node(node_id, refpath.clone(), node_length, offset);
        }
        // For all nodes except the first one, set the previous node as a parent
        if let Some(parent) = parent_node.clone() {
            tree.add_child(parent.id, node_id);
        }
        parent_node = Some(tree.nodes.get(&node_id).unwrap().clone());
        offset += node_length;
    }

    // Remove the reference path from the paths
    paths.remove(reference);

    // We add all the other paths to the tree
    for (path_name, node_list) in paths.iter() {
        path_index += 1;
        let path: Path = Path::new(path_name.to_string(), path_index);
        parent_node = None;
        // If the node is not in the tree, we add it only if there is a parent
        // If the node is in the tree, and it exists a parent, we add the node as a child
        for node_name in node_list {
            let node_id: i32 = node_name.parse().unwrap();
            let node_length: i32 = sequence_lengths.get(&node_id).unwrap().clone();
            // We add the node if it is not already in the tree
            if !tree.nodes.contains_key(&node_id) && parent_node.is_some() {
                tree.add_node(
                    node_id,
                    path.clone(),
                    node_length,
                    parent_node.as_ref().unwrap().offset + parent_node.as_ref().unwrap().length,
                );
            }
            // For all nodes except the first one, set the previous node as a parent
            if let Some(parent) = parent_node.clone() {
                tree.add_child(parent.id, node_id);
            }
            // If the node is in the tree, it becomes the new parent
            if tree.nodes.contains_key(&node_id) {
                parent_node = Some(tree.nodes.get(&node_id).unwrap().clone());
            }
        }
    }

    // We read the file a second time to print the rGFA version
    reader.seek(io::SeekFrom::Start(0))?;
    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'S' {
                let node_id: i32 = columns[1].parse().unwrap();
                if tree.nodes.contains_key(&node_id) {
                    let sn: String = tree.get_node(node_id).get_path().get_name().clone();
                    let so: i32 = tree.get_node(node_id).get_offset();
                    let sr: i32 = tree.get_node(node_id).get_path().get_index();
                    // In the case of an S-line, we add the SN, SO, and SR fields
                    println!("{}\tSN:Z:{}\tSO:i:{}\tSR:i:{}", line.trim_end(), sn, so, sr);
                } else {
                    // In the case of an S-line, we print without modification
                    println!("{}", line.trim_end());
                }
            }
            if first_char == 'E' {
                // In the case of a E-line, we print without modification
                println!("{}", line.trim_end());
            }
            // In any other case, we don't print the line
        }
        line.clear(); // Clear the line buffer for the next read
    }

    Ok(())
}
