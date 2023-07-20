use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author = "Davide Monti <d.monti11@campus.unimib.it>", version, about = "RecGraph", long_about = None)]
struct Args {
    #[clap(
        help_heading = "I/O",
        help = "Input sequence (in .fasta format)",
        required = true
    )]
    sequence_path: String,
    #[clap(
        help_heading = "I/O",
        help = "Input graph (in .gfa format)",
        required = true
    )]
    graph_path: String,

    // Alignment mode
    #[clap(
        help_heading = "Parameters",
        short = 'k',
        long = "k-len",
        default_value_t = 5,
        help = "Set k-mer length"
    )]
    kmer_length: i32,
}

pub fn get_kmer_length() -> i32 {
    let args = Args::parse();
    args.kmer_length
}

pub fn get_sequence_path() -> String {
    let args = Args::parse();
    args.sequence_path
}

pub fn get_graph_path() -> String {
    let args = Args::parse();
    args.graph_path
}
