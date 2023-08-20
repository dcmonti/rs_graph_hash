use rs_graph_hash::cli;
use rs_graph_hash::io_parser;
use rs_graph_hash::kmers_extraction;
use rs_graph_hash::kmers_match;
use std::time::Instant;

fn main() {
    // Parse args
    let k = cli::get_kmer_length() as usize;
    let graph_path = cli::get_graph_path();
    let read_path = cli::get_sequence_path();
    let amb_mode = cli::get_amb_mode();
    let _rec_mode = cli::get_rec_mode();

    let reads = io_parser::read_sequence_w_path(&read_path, amb_mode);
    let graph = io_parser::read_graph_w_path(&graph_path);

    // Extract graph's unique k-mers
    let start = Instant::now();
    let unique_kmers = kmers_extraction::extract_unique_kmers(&graph, k);
    let end = start.elapsed().as_micros();
    println!("extr PAR: {end}");

    // Find possible recombinations
    for (id, read) in reads {
        let start = Instant::now();
        let seeds = kmers_match::match_read_kmers(&read, &unique_kmers, k);
        let end = start.elapsed().as_micros();
        println!("match: {end}");

        io_parser::output_formatter(&seeds, &id)
    }
}
