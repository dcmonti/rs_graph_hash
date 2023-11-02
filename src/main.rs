use rayon::prelude::*;
use rs_graph_hash::cli;
use rs_graph_hash::dump;
use rs_graph_hash::extract_subgraph;
use rs_graph_hash::io_parser;
use rs_graph_hash::kmers_extraction;
use rs_graph_hash::kmers_match;
use std::panic;

fn main() {
    let result = panic::catch_unwind(|| {
        // Parse args
        let graph_path = cli::get_graph_path();
        let read_path = cli::get_sequence_path();

        let k = cli::get_kmer_length();
        let amb_mode = cli::get_amb_mode();
        let mode = cli::get_mode();
        let base_skip = cli::get_base_skip();
        let seed_merge = cli::get_seed_merge();

        match mode {
            0 => {
                let reads = io_parser::read_sequence_w_path(&read_path, amb_mode);
                let graph = io_parser::read_graph_w_path(&graph_path);

                // Extract graph's unique k-mers
                let unique_kmers = kmers_extraction::extract_unique_kmers(&graph, k);

                // Parallel kmer match
                reads.par_iter().for_each(|(id, read)| {
                    let seeds = kmers_match::match_read_kmers(
                        &read,
                        &unique_kmers,
                        k,
                        base_skip,
                        seed_merge,
                    );
                    // TODO:
                    //  estrazione precisa della sottostringa, path nel subgraph?
                    //  EXAMPLE: 118 136
                    if seeds.len() >= 2 {
                        for i in 0..seeds.len() - 1 {
                            let this_seed = seeds.get(i).unwrap();
                            let next_seed = seeds.get(i + 1).unwrap();
                            let sub = extract_subgraph::execute(
                                &graph,
                                this_seed.positions[1].node_id,
                                this_seed.positions[1].offset,
                                next_seed.positions[0].node_id,
                                next_seed.positions[0].offset,
                            );
                            let subread = read
                                [this_seed.positions[3].offset..next_seed.positions[2].offset]
                                .to_string();
                            let res = recgraph::api::align_local_no_gap(&subread, &sub, None, None);
                            println!("{}", res.to_string());
                        }
                    }

                    io_parser::output_formatter(&seeds, &id);
                });
            }
            1 => {
                let reads = io_parser::read_sequence_w_path(&read_path, amb_mode);
                let (unique_kmers, k) = dump::load_unique_kmers(&graph_path);

                // Parallel kmer match.
                reads.par_iter().for_each(|(id, read)| {
                    let seeds = kmers_match::match_read_kmers(
                        &read,
                        &unique_kmers,
                        k,
                        base_skip,
                        seed_merge,
                    );
                    io_parser::output_formatter(&seeds, &id);
                });
            }
            2 => {
                let graph = io_parser::read_graph_w_path(&graph_path);
                let unique_kmers = kmers_extraction::extract_unique_kmers(&graph, k);
                let out_file = cli::get_out_file();
                if out_file == "standard output" {
                    panic!("output file for .dmp must be specified")
                }
                dump::dump_unique_kmers(&unique_kmers, k, &out_file);
            }

            _ => {
                panic!("invalid mode")
            }
        }
    });

    if let Err(err) = result {
        eprintln!("An error occurred: {:?}", err);
        std::process::exit(1);
    }
}
