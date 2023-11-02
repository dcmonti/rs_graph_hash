use handlegraph::hashgraph::Path;
use rayon::prelude::*;
use rustc_hash::FxHashMap as HashMap;
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
    >::default()));
    let found_kmers = Arc::new(Mutex::new(
        HashMap::<String, (Coordinate, Coordinate)>::default(),
    ));

    // parallel extraction
    paths.par_iter().for_each(|path| {
        let node_labels: Vec<String> = path
            .nodes
            .iter()
            .map(|handle| String::from_utf8(graph.sequence(*handle)).unwrap())
            .collect();
        let path_seq = node_labels.join("");

        let path_nodes_id: Vec<u64> = path
            .nodes
            .iter()
            .flat_map(|handle| vec![handle.id().into(); graph.sequence(*handle).len()])
            .collect();

        for i in 0..path_seq.len() - k_len + 1 {
            let kmer = &path_seq[i..i + k_len];

            let kmer_start = Coordinate::build(path_nodes_id[i], i, &path_nodes_id);
            let kmer_end =
                Coordinate::build(path_nodes_id[i + k_len - 1], i + k_len - 1, &path_nodes_id);

            let mut unique_kmers_locked = unique_kmers.lock().unwrap();
            let mut found_kmers_locked = found_kmers.lock().unwrap();

            if let Some((found_kmer_start, found_kmer_end)) = found_kmers_locked.get(kmer) {
                if let Some((_, _, paths)) = unique_kmers_locked.get_mut(kmer) {
                    if kmer_start.equal(found_kmer_start) && kmer_end.equal(found_kmer_end) {
                        paths.set(path.path_id as usize, true);
                    } else {
                        // println!("Repeated kmer: [{}]:{}\t[{}]:{}", kmer_start.node_id, kmer_start.offset, kmer_end.node_id, kmer_end.offset);
                        unique_kmers_locked.remove(kmer);
                    }
                }
            } else {
                found_kmers_locked.insert(kmer.to_string(), (kmer_start, kmer_end));

                let mut kmer_paths = BitVec::from_elem(paths_number, false);
                kmer_paths.set(path.path_id as usize, true);
                unique_kmers_locked.insert(kmer.to_string(), (kmer_start, kmer_end, kmer_paths));
            }
        }
    });

    Arc::try_unwrap(unique_kmers).unwrap().into_inner().unwrap()
}
