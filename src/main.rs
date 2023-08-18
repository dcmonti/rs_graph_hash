use std::time::Instant;

use rs_graph_hash::cli;
use rs_graph_hash::io_parser;
use rs_graph_hash::k_mers_extraction;
use rs_graph_hash::k_mers_match;
use rs_graph_hash::new_kmers_match;

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
        let start = Instant::now();
        let recombs = k_mers_match::find_recomb_kmers(&read, &unique_kmers, k, rec_mode);
        let end = start.elapsed().as_millis();
        io_parser::output_formatter(&recombs, &graph, &id);

        println!("OLD: {end}");
        let start = Instant::now();
        let seeds = new_kmers_match::match_read_kmers(&read, &unique_kmers, k);
        let end = start.elapsed().as_millis();
        println!("NEW: {end}");

        for seed in seeds.iter() {
            println!("{:#?}", seed)
        }
    }
}
