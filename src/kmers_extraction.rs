use handlegraph::hashgraph::Path;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use bit_vec::BitVec;
use handlegraph::{handlegraph::HandleGraph, hashgraph::HashGraph};

use crate::coordinate::Coordinate;

pub fn extract_unique_kmers(
    graph: &HashGraph,
    k_len: usize,
) -> HashMap<String, (Coordinate, Coordinate, BitVec)> {
    // extract paths from graph
    let paths = graph.paths.values().collect::<Vec<&Path>>();
    let paths_number = paths.len();

    // init hashmaps to store result
    let unique_kmers = Arc::new(Mutex::new(HashMap::<
        String,
        (Coordinate, Coordinate, BitVec),
    >::new()));
    let found_kmers = Arc::new(Mutex::new(
        HashMap::<String, (Coordinate, Coordinate)>::new(),
    ));

    // parallel extraction
    paths.par_iter().for_each(|path| {
        let mut path_seq = String::new();
        let mut path_nodes_id: Vec<u64> = Vec::new();

        for handle in path.nodes.iter() {
            let handle_seq = String::from_utf8(graph.sequence(*handle)).unwrap();
            path_seq.push_str(&handle_seq);
            path_nodes_id.append(&mut vec![handle.id().into(); handle_seq.len()]);
        }

        for i in 0..path_seq.len() - k_len + 1 {
            let kmer: String = path_seq.chars().skip(i).take(k_len).collect();
            let kmer_start = Coordinate::build(path_nodes_id[i], i, &path_nodes_id);
            let kmer_end =
                Coordinate::build(path_nodes_id[i + k_len - 1], i + k_len - 1, &path_nodes_id);

            let mut unique_kmers_locked = unique_kmers.lock().unwrap();
            let mut found_kmers_locked = found_kmers.lock().unwrap();

            if let Some((found_kmer_start, found_kmer_end)) = found_kmers_locked.get(&kmer) {
                if let Some((_, _, paths)) = unique_kmers_locked.get_mut(&kmer) {
                    if kmer_start.equal(found_kmer_start) && kmer_end.equal(found_kmer_end) {
                        paths.set(path.path_id as usize, true);
                    } else {
                        unique_kmers_locked.remove(&kmer);
                    }
                }
            } else {
                found_kmers_locked.insert(kmer.clone(), (kmer_start, kmer_end));

                let mut kmer_paths = BitVec::from_elem(paths_number, false);
                kmer_paths.set(path.path_id as usize, true);
                unique_kmers_locked.insert(kmer, (kmer_start, kmer_end, kmer_paths));
            }
        }
    });

    Arc::try_unwrap(unique_kmers).unwrap().into_inner().unwrap()
}
