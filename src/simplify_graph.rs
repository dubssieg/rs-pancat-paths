// Simplify the graph by merging and creating nodes and edges.
// We need to load the graph in memory (at least its structure) and then we can simplify it.
// We can then write the simplified graph to a new file.
// Targets to simplify: subsitiution nodes, loops, spurious breakpoints.
use petgraph::graph::Graph;
use petgraph::prelude::{Directed, NodeIndex};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn simplify_graph(file_path: &str) {
    let graph: Graph<String, String> = match load_graph(file_path) {
        Ok(graph) => graph,
        Err(e) => {
            eprintln!("Error loading graph: {}", e);
            return;
        }
    };

    simp_substitution(graph);
    simp_loops();
    simp_spurious();
    write_graph(file_path);
}

fn load_graph(file_path: &str) -> Result<Graph<String, String>, io::Error> {
    let mut backbone: Graph<String, String, Directed> = Graph::new();

    let file: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(file);

    let mut line: String = String::new();
    while reader.read_line(&mut line)? > 0 {
        let columns: Vec<&str> = line.split('\t').collect();
        if let Some(first_char) = line.chars().next() {
            if first_char == 'E' {
                // In the case of an E-line, we store the predecessor and successor nodes
                let predecessor: NodeIndex = backbone.add_node(String::from(columns[1]));
                let successor: NodeIndex = backbone.add_node(String::from(columns[3]));
                backbone.add_edge(predecessor, successor, String::from(columns[1..4].concat()));
            }
            line.clear(); // Clear the line buffer for the next read
        }
    }

    Ok(backbone)
}

fn write_graph(file_path: &str) {}

fn simp_substitution(graph: Graph<String, String>) {}

fn simp_loops() {}

fn simp_spurious() {}
