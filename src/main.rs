mod anchor;
mod concatenate;
mod converter;
mod index_gfa_file;
mod mask_paths;
mod optimize;
mod reconstruct;
mod remove_loops;
mod sharepg;
mod simplify_graph;
mod spurious;
use clap::{Parser,Subcommand};

#[derive(Parser, Debug)]
#[command(version = "v0.1.0", about, long_about = None)]
struct Cli {
    /// The path to the GFA file
    file_path: String,

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Rename the paths to the names given in the file.
    Rename {
        /// Path to tab-separated file with old names and new names for paths
        #[arg(short = 'r', long = "rename")]
        rename_file: String,
    },
    /// Computes shared parts of a pangenome
    Share {
        /// Query intervals that are included in the given paths
        #[arg(short = 'i', long = "include")]
        include: Vec<String>,
        /// Query intervals that are excluded from the given paths
        #[arg(short = 'e', long = "exclude")]
        exclude: Vec<String>,
        /// Sensitivity ratio for the query intervals (from 0.0 to 1.0)
        #[arg(short = 's', long = "sensitivity", default_value_t = 1.0)]
        sensitivity: f64,
    },
    /// Convert to rGFA using the reference as a backbone for the offset tree.
    Convert {
        /// Name of reference path in graph, used to construct rgfa tree
        #[arg(short = 'R', long = "rgfa")]
        rgfa_reference: String,
    },
    /// Search for anchor nodes
    Anchors {
        /// Minimum number of crossing distinct haplotypes to consider a node as an anchor
        #[arg(short = 'a', long = "anchor")]
        anchor: Option<i32>,
    },
    /// Computes offsets of the nodes in the graph
    Offsets {},
    /// Computes a simplified version of the graph
    Simplify {},
    /// Computes spurious breakpoints in the graph
    Spurious {},
    /// Computes lengths of the nodes in the graph
    Lengths {},
    /// Reconstruct paths from the graph
    Reconstruct {},
    /// Computes a simplified version of the graph
    Loops {},
    /// Filter the paths to be removed from the graph
    Mask {
        /// Paths names to be removed
        #[clap(long = "mask", short = 'M', action)]
        mask: Vec<String>,
    },
    /// Optimize the graph, reallocating IDs of the nodes
    Opitmize {
        /// Location to store mapping between old and new series of node IDs
        #[arg(short = 'O', long = "optimize")]
        output_mapping: String,
    },
    /// Concatenate graph with a second one, keeping tags.
    Concatenate {
        /// Path to second GFA to concatenate with the first one
        #[arg(short = 'c', long = "concat")]
        graph_to_concat: String,
    },
    /// Retrieve basic information about the paths of the graph
    Index {},
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

    match &args.cmd {
        Commands::Rename { rename_file } => {
            let _ = index_gfa_file::rename_paths(&args.file_path, &rename_file);
        }
        Commands::Share { include, exclude, sensitivity } => {
            let _ = sharepg::shared_nodes(
                &args.file_path,
                &include,
                &exclude,
                *sensitivity,
            );
        }
        Commands::Convert { rgfa_reference } => {
            let _ = converter::gfa_to_rgfa(&args.file_path, &rgfa_reference);
        }
        Commands::Anchors { anchor } => {
            let _ = anchor::anchor_nodes(&args.file_path, *anchor);
        }
        Commands::Offsets { } => {
            let _ = index_gfa_file::offset_gfa(&args.file_path);

        }
        Commands::Simplify { } => {
            let _ = simplify_graph::simplify_graph(&args.file_path);

        }
        Commands::Spurious { } => {
            let _ = spurious::prune_spurious_breakpoints(&args.file_path);

        }
        Commands::Lengths { } => {
            let _ = index_gfa_file::lengths_gfa(&args.file_path);

        }
        Commands::Reconstruct { } => {
            let _ = reconstruct::reconstruct_paths(&args.file_path);

        }
        Commands::Loops { } => {
            let _ = remove_loops::remove_loops(&args.file_path, 2);

        }
        Commands::Mask { mask } => {
            let _ = mask_paths::mask_paths(
                &args.file_path,
                mask.iter().map(String::as_str).collect(),
            );
        }
        Commands::Opitmize { output_mapping } => {
            let _ = optimize::relocate_ids(&args.file_path, output_mapping);

        }
        Commands::Concatenate { graph_to_concat } => {
            let _ = concatenate::concat_graphs(&args.file_path, graph_to_concat);

        }
        Commands::Index {  } => {
            let _ = index_gfa_file::index_gfa(&args.file_path);
        }
    }

}
