use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    author = "Davide Monti <d.monti11@campus.unimib.it>\nNada Mohamed <n.mohamed3@campus.unimib.it>",
    version,
    about = "rs_graph_hash",
    long_about = "Alignment free sequence to graph"
)]
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

    #[clap(
        help_heading = "Parameters",
        short = 'k',
        long = "k-len",
        default_value_t = 5,
        help = "Set k-mer length"
    )]
    kmer_length: i32,

    #[clap(
        help_heading = "Parameters",
        short = 'r',
        long = "rec-mode",
        default_value_t = 0,
        help = "Set recombination selection mode:\n\tIf 0 consider every k-mers\n\tIf 1 consider only consecutive ones"
    )]
    rec_mode: i32,

    //Ambiguos strand mode
    #[clap(
        help_heading = "Parameters",
        short = 's',
        long = "amb-strand",
        default_value_t = 0,
        help = "Set ambigous strand mode:\n\tIf 0 use input sequence\n\tIf 1 try also align with rev & compl"
    )]
    amb_strand: i32,

    #[clap(
        help_heading = "I/O",
        short = 'o',
        long = "out_file",
        default_value = "standard output",
        help = "Output reombinations file"
    )]
    out_file: String,
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

pub fn get_amb_mode() -> bool {
    let args = Args::parse();
    let strand_mode = args.amb_strand;
    strand_mode != 0
}

pub fn get_out_file() -> String {
    let args = Args::parse();
    args.out_file
}

pub fn get_rec_mode() -> i32 {
    let args = Args::parse();
    args.rec_mode
}
