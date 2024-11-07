mod index_gfa_file;
use std::env;

fn main() {
    /*
    Compare two GFA files and compute the distance between them
    Two arguments must be given in the command line:
    - The path to the first GFA file
    - The path to the second GFA file
    It will print to standard output the differences between the two graphs
    */
    // Get the file path from command line arguments
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // Parse first graph
    let () = match index_gfa_file::index_gfa(file_path) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("Failed to read GFA file: {}", error);
            return;
        }
    };
}
