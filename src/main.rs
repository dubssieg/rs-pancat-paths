mod index_gfa_file;
mod sharepg;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "v0.1.0", about, long_about = None)]
struct Cli {
    /// The path to the GFA file
    file_path: String,
    /// Rename the paths to the names given in the file.
    #[arg(short = 'r', long = "rename")]
    rename_file: Option<String>,
    /// Query intervals that are included in the given paths
    #[arg(short = 'i', long = "include")]
    include: Vec<String>,
    /// Query intervals that are excluded from the given paths
    #[arg(short = 'e', long = "exclude")]
    exclude: Vec<String>,
    /// Sensitivity ratio for the query intervals (from 0.0 to 1.0)
    #[arg(short = 's', long = "sensitivity", default_value_t = 1.0)]
    sensitivity: f64,
}

fn main() {
    /*
    Gets the length and info on each path of the graph
    One argument must be given in the command line:
    - The path to the GFA file
    It will print to standard information about the paths
    */

    // Default behavior: prints the names of the path and information about the paths

    // Options:
    // -r, --rename: rename the paths to the names given in the file.
    // File is a .tsv file with two columns: old_name and new_name

    // Get the file path from command line arguments
    let args: Cli = Cli::parse();

    if let Some(rename_file) = &args.rename_file {
        let _ = index_gfa_file::rename_paths(&args.file_path, rename_file);
    } else if (args.include.len() > 0) || (args.exclude.len() > 0) {
        let _ = sharepg::shared_nodes(
            &args.file_path,
            &args.include,
            &args.exclude,
            args.sensitivity,
        );
    } else {
        // If the rename option is not given, index the paths
        let _ = index_gfa_file::index_gfa(&args.file_path);
    }
}
