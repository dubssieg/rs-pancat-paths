mod anchor;
mod converter;
mod index_gfa_file;
mod mask_paths;
mod remove_loops;
mod sharepg;
mod simplify_graph;
mod spurious;
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
    /// Convert to rGFA using the reference as a backbone for the offset tree.
    #[arg(short = 'R', long = "rgfa")]
    rgfa_reference: Option<String>,
    /// Query intervals that are excluded from the given paths
    #[arg(short = 'e', long = "exclude")]
    exclude: Vec<String>,
    /// Sensitivity ratio for the query intervals (from 0.0 to 1.0)
    #[arg(short = 's', long = "sensitivity", default_value_t = 1.0)]
    sensitivity: f64,
    /// Search for anchor nodes
    #[arg(short = 'a', long = "anchor")]
    anchor: Option<i32>,
    /// Computes offsets of the nodes in the graph
    #[clap(long = "index", short = 'I', action)]
    index: bool,
    /// Computes a simplified version of the graph
    #[clap(long = "simplify", short = 'S', action)]
    simplify: bool,
    /// Computes spurious breakpoints in the graph
    #[clap(long = "spurious", short = 'B', action)]
    spurious: bool,
    /// Computes lengths of the nodes in the graph
    #[clap(long = "lengths", short = 'L', action)]
    lengths: bool,
    /// Computes a simplified version of the graph
    #[clap(long = "loops", short = 'l', action)]
    loops: bool,
    /// Filter the paths to be removed from the graph
    #[clap(long = "mask", short = 'M', action)]
    mask: Vec<String>,
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
    } else if args.anchor.is_some() {
        let _ = anchor::anchor_nodes(&args.file_path, args.anchor);
    } else if let Some(rgfa_reference) = &args.rgfa_reference {
        let _ = converter::gfa_to_rgfa(&args.file_path, rgfa_reference);
    } else if args.index {
        let _ = index_gfa_file::offset_gfa(&args.file_path);
    } else if args.lengths {
        let _ = index_gfa_file::lengths_gfa(&args.file_path);
    } else if args.simplify {
        let _ = simplify_graph::simplify_graph(&args.file_path);
    } else if args.spurious {
        let _ = index_gfa_file::find_spurious_breakpoints(&args.file_path);
    } else if args.loops {
        let _ = remove_loops::remove_loops(&args.file_path, 2);
    } else if !args.mask.is_empty() {
        let _ = mask_paths::mask_paths(
            &args.file_path,
            args.mask.iter().map(String::as_str).collect(),
        );
    } else {
        // If the rename option is not given, index the paths
        let _ = index_gfa_file::index_gfa(&args.file_path);
    }
}
