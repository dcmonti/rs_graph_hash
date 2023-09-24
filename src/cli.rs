use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    author = "Davide Monti <d.monti11@campus.unimib.it>",
    version = "0.1.0",
    about = "rs_graph_hash",
    long_about = "Alignment free sequence to graph"
)]
struct Args {
    #[clap(
        help_heading = "I/O",
        help = "Input graph (in .gfa format or dump file)",
        default_value = "none",
        required = true
    )]
    graph_path: String,

    #[clap(
        help_heading = "I/O",
        help = "Input sequence (in .fasta format)",
        default_value = "none"
    )]
    sequence_path: String,

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
        short = 'm',
        long = "mode",
        default_value_t = 0,
        help = "Set execution mode:\n\t0: find matching kmers between a read (.fasta) and a graph (.gfa)\n\t1: find matching kmers between a read (.fasta) and a .dmp file\n\t2: extract and save in a .dmp file a graph's unique kmers"
    )]
    mode: i32,

    #[clap(
        help_heading = "Parameters",
        short = 'b',
        long = "skip-base",
        default_value_t = 1,
        help = "Set bp to skip after matching kmers couple"
    )]
    base_skip: i32,

    #[clap(
        help_heading = "Parameters",
        short = 's',
        long = "seed-merge",
        default_value_t = 0,
        help = "Set seed merge mode:\n\tIf 0 merge seed if they cover a common subportion of the graph\n\tIf 1 merge seed if they have common paths"
    )]
    seed_merge: i32,

    //Ambiguos strand mode
    #[clap(
        help_heading = "Parameters",
        short = 'a',
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
        help = "Output file path"
    )]
    out_file: String,
}

pub fn get_kmer_length() -> usize {
    let args = Args::parse();
    args.kmer_length as usize
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

pub fn get_mode() -> i32 {
    let args = Args::parse();
    args.mode
}

pub fn get_base_skip() -> usize {
    let args = Args::parse();
    args.base_skip as usize
}

pub fn get_seed_merge() -> i32 {
    let args = Args::parse();
    let seed_merge = args.seed_merge;
    if seed_merge == 0 || seed_merge == 1 {
        seed_merge
    } else {
        panic!("seed merge mode must be 0 or 1")
    }
}
