# rs_graph_hash
`rs_graph_hash` is an identifier of possible recombination sites, exploiting unique k-mers location in a variation graph.

Receiving a `.gfa` file (representing a variation graph) and a `.fa` file (representing the sequence) as inputs, it proceeds to extract the unique k-mers positions from the graph (i.e. k-mers that appear only once in the whole graph) and then tries to find matching k-mers between them and the read's ones.
Finally, each matching k-mer's paths are compared with those of the remaining k-mers. If there are no common paths, the k-mers pair is considered a sign of a possible recombination.


## Installation
Install [`rust`](https://doc.rust-lang.org/cargo/getting-started/installation.html), then clone and install `rs_graph_hash`:
```
git clone https://github.com/dcmonti/rs_graph_hash
cd rs_graph_hash
cargo build --release
```

## Usage
`rs_graph_hash` requires as input a variation graph in `.gfa` format and a set of sequences (reads) in `.fasta`. To run `rs_graph_hash` type:
```
target/release/rs_hash_graph <reads.fa> <graph.gfa>
```

## Parameters
`rs_graph_hash` allows to set multiple parameters to tweak the execution. Here the list of parameters (please check also `--help`): 

```
    Options:
        -h, --help                      Print help
        -V, --version                   Print version

    I/O:
        -o, --out_file <OUT_FILE>       Output reombinations file [default: "standard output"]
        <GRAPH_PATH>                    Input graph (in .gfa format)
        <SEQUENCE_PATH>                 Input sequence (in .fasta format)

    Parameters:
        -k, --k-len <KMER_LENGTH>
            Set k-mer length

            [default: 5]

        -r, --rec-mode <REC_MODE>
            Set recombination selection mode:
                If 0 consider every k-mers
                If 1 consider only consecutive ones
                if 2 consider only one unique k-mer for each position

            [default: 0]

        -s, --amb-strand <AMB_STRAND>
            Set ambigous strand mode:
                If 0 use input sequence
                If 1 try also align with rev & compl

            [default: 0]
```

## Output

`rs_graph_hash` returns a `.rec` file if specified. It's a tab separated format, where each record is a possible recombination site made by 9 fields:

1. Sequence id
2. First k-mer starting position (graph)
3. First k-mer ending position (graph)
4. First k-mer paths
5. First k-mer starting position (read)
6. Second k-mer starting position (graph)
7. Second k-mer ending position (graph)
8. Second k-mer paths
9. Second k-mer starting position (read)

