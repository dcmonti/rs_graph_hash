use core::panic;

use rs_graph_hash::cli;
use rs_graph_hash::dump;
use rs_graph_hash::io_parser;
use rs_graph_hash::kmers_extraction;
use rs_graph_hash::kmers_match;

fn main() {
    // Parse args
    let k = cli::get_kmer_length() as usize;
    let graph_path = cli::get_graph_path();
    let read_path = cli::get_sequence_path();
    let amb_mode = cli::get_amb_mode();
    let mode = cli::get_mode();
    let base_skip = cli::get_base_skip();

    match mode {
        0 => {
            let reads = io_parser::read_sequence_w_path(&read_path, amb_mode);
            let graph = io_parser::read_graph_w_path(&graph_path);

            // Extract graph's unique k-mers
            let unique_kmers = kmers_extraction::extract_unique_kmers(&graph, k);

            // Find possible recombinations
            for (id, read) in reads {
                let seeds = kmers_match::match_read_kmers(&read, &unique_kmers, k, base_skip);
                io_parser::output_formatter(&seeds, &id)
            }
        }
        1 => {
            let reads = io_parser::read_sequence_w_path(&read_path, amb_mode);
            let (unique_kmers, k) = dump::load_unique_kmers(&graph_path);

            // Find possible recombinations
            for (id, read) in reads {
                let seeds = kmers_match::match_read_kmers(&read, &unique_kmers, k, base_skip);
                io_parser::output_formatter(&seeds, &id)
            }
        }
        2 => {
            let graph = io_parser::read_graph_w_path(&graph_path);
            let unique_kmers = kmers_extraction::extract_unique_kmers(&graph, k);
            let out_file = cli::get_out_file();
            if out_file == "standard output" {
                panic!("output file for .dmp must be specified")
            }
            dump::dump_unique_kmers(&unique_kmers, k, &out_file)
        }
        _ => {}
    }
}
