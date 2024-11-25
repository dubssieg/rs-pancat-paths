# Pangenome graph paths manipulation tool

> [!NOTE]\
>  [pancat paths](https://github.com/dubssieg/pancat) is a tool, originally written in Python, designed to extract informations from the paths of a graph. For performance, it has been reimplemented in Rust, bundled together with [sharepg](https://github.com/dubssieg/sharepg).

Offer tools to manipulate the paths of your GFA, including:
+ Detailled statistics
+ Renaming of paths
+ Analysis of shared subpaths

## Install instructions:

Requires rust and cargo.

```bash
git clone 
cd rs-pancat-paths
cargo build --release
```

## Usage

### Extract path stats:

```bash
rs-pancat-paths graph.gfa > output.tsv
```

The following stats will be saved (one line per path):
+ name
+ length
+ length in forward orientation
+ length in reverse orientation

### Rename paths in GFA:

```bash
rs-pancat-paths graph.gfa -r rename_file.txt > output.gfa
```

The file `rename_file.txt` must be a file containing per line the old name (present in the graph) and the new name (to be remplaced with) separated by `\t`.

### Computes shared regions:

```bash
rs-pancat-paths graph.gfa -i path_x [...] -e path_y [...] -s 0.95 > output.tsv
```

Extracts presence-absence of paths crossing nodes of the graph as presence vectors. Computes depending on a sensitivity ratio if a region is shared by a group of genomes `-i` with at least `-s` of paths crossing them and if a region is not crossed by a group `-e` of genomes with at most 1-`-s` paths crossing them.

> [!NOTE]\
> Want to contribute? Feel free to open a PR on an issue about a missing, buggy or incomplete feature! **Please do bug reports in the issue tracker!**.

### GFA1 to rGFA

```bash
rs-pancat-paths graph.gfa -R reference > output.gfa
```

Builds a offset tree using `reference` as backbone, and uses it to compute rGFA supplementary tags.


### Anchor nodes

Anchor nodes are nodes that are shared by at least $n$ paths. Anchor rank is the number of paths crossing a single node (whitout cycles)
It is helpful to find candidates sources and sink, pivot points, tree roots, highly conserved regions...

```bash
rs-pancat-paths graph.gfa -a n > output.tsv
```