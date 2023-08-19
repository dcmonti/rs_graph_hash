use std::collections::HashMap;

use bit_vec::BitVec;
use handlegraph::{
    handle::Handle,
    handlegraph::HandleGraph,
    hashgraph::HashGraph,
};

use crate::coordinate::Coordinate;

pub fn extract_unique_kmers(graph: &HashGraph, k_len: usize) -> HashMap<String, (Coordinate, Coordinate, BitVec)> {

    let paths_set = &graph.paths;

    let mut paths = Vec::new();
    for (_id, path) in paths_set.iter() {
        paths.push(path)
    }
    for (id, path) in paths_set.iter() {
        paths[*id as usize] = path
    }

    let paths_number = paths_set.keys().len();

    let mut found_kmers = HashMap::new();
    let mut unique_kmers = HashMap::new();

    for (path_id, path) in paths.iter().enumerate() {
        let path_nodes = path.nodes.iter().collect::<Vec<&Handle>>();
        let mut path_seq = String::new();
        let mut path_node_id: Vec<u64> = Vec::new();
        
        for handle in path_nodes {
            let handle_seq = String::from_utf8(graph.sequence(*handle)).unwrap();
            path_seq.push_str(&handle_seq);
            for _ in 0..handle_seq.len() {
                path_node_id.push(handle.id().into())
            }
        }
        for i in 0..path_seq.len() - k_len + 1 {
            let kmer: String = path_seq.chars().skip(i).take(k_len).collect();
            let kmer_start = Coordinate::build(path_node_id[i], i, &path_node_id);
            let kmer_end = Coordinate::build(path_node_id[i+k_len-1], i+k_len-1, &path_node_id);

            if found_kmers.contains_key(&kmer) {
                let (found_kmer_start, found_kmer_end) = found_kmers.get(&kmer).unwrap();
                if unique_kmers.contains_key(&kmer) && kmer_start.equal(found_kmer_start) && kmer_end.equal(found_kmer_end) {
                    let (_, _, paths): &mut (Coordinate, Coordinate, BitVec) = unique_kmers.get_mut(&kmer).unwrap();
                    paths.set(path_id, true)
                } else {
                    unique_kmers.remove(&kmer);
                }
            } else {
                found_kmers.insert(kmer.clone(), (kmer_start.clone(), kmer_end.clone()));

                let mut kmer_paths = BitVec::from_elem(paths_number, false);
                kmer_paths.set(path_id, true);
                unique_kmers.insert(kmer, (kmer_start, kmer_end, kmer_paths));
            }

        }
    }
   unique_kmers
}
