use rs_graph_hash::cli;
use rs_graph_hash::io_parser;
use rs_graph_hash::k_mers_extraction;
use rs_graph_hash::k_mers_match;

fn main() {
    // Parse args
    let k = cli::get_kmer_length() as usize;
    let graph_path = cli::get_graph_path();
    let read_path = cli::get_sequence_path();
    let amb_mode = cli::get_amb_mode();
    let rec_mode = cli::get_rec_mode();

    let graph = io_parser::read_graph_w_path(&graph_path);
    let reads = io_parser::read_sequence_w_path(&read_path, amb_mode);

    // Extract graph's unique k-mers
    let unique_kmers = k_mers_extraction::extract_graph_unique_kmers(&graph, k);

    // Find possible recombinations
    for (id, read) in reads {
        let recombs = k_mers_match::find_recomb_kmers(&read, &unique_kmers, k, rec_mode);
        io_parser::output_formatter(&recombs, &graph, &id);
    }
}
