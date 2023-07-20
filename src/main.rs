use rs_graph_hash::io_parser;
use rs_graph_hash::k_mers_extraction;
use rs_graph_hash::k_mers_extraction::find_recomb_kmers;

fn main() {
    let k = 18;
    let graph = io_parser::read_graph_w_path("./graph.gfa", false);
    let read = io_parser::read_sequence_w_path("./read.fa");
    let unique_kmers = k_mers_extraction::extract_graph_unique_kmers(&graph, k);
    let candidates_kmers = k_mers_extraction::filter_read_kmers(&read, &unique_kmers, k);
    
    find_recomb_kmers(&candidates_kmers);
    
    /*
    for (k, v) in candidates_kmers.iter() {
        println!("POS: {:?}\tPATHS: {:?}", k, v);
    }
     */
    

    //graph.to_string();
}
