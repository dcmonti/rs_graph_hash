use rs_graph_hash::cli;
use rs_graph_hash::io_parser;
use rs_graph_hash::k_mers_extraction;
use rs_graph_hash::k_mers_extraction::find_recomb_kmers;

fn main() {
    let k = cli::get_kmer_length() as usize;
    let graph_path = cli::get_graph_path();
    let read_path = cli::get_sequence_path();

    let graph = io_parser::read_graph_w_path(&graph_path, false);
    let read = io_parser::read_sequence_w_path(&read_path);
    let unique_kmers = k_mers_extraction::extract_graph_unique_kmers(&graph, k);

    println!("UNIQUE KMERS");
    for (k, v) in unique_kmers.iter() {
        println!("KMER: {:?}\tPATHS: {:?}", k, v);
    }

    let candidates_kmers = k_mers_extraction::filter_read_kmers(&read, &unique_kmers, k);
    println!("");
    println!("CANDIDATES KMERS");
    for (k, v) in candidates_kmers.iter() {
        println!("POS: {:?}\tPATHS: {:?}", k, v);
    }
    find_recomb_kmers(&candidates_kmers);

    /*
    for (k, v) in candidates_kmers.iter() {
        println!("POS: {:?}\tPATHS: {:?}", k, v);
    }
     */

    //graph.to_string();
}
