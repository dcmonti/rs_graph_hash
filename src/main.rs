use rs_graph_hash::graph_parser;
use rs_graph_hash::k_mers_extraction;
use std::collections::VecDeque;

fn main() {
    let graph = graph_parser::read_graph_w_path("./prova.gfa", false);

    k_mers_extraction::extract_kmers(&graph, 3);
    //graph.to_string();

    
}
