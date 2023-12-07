use fxhash::hash;
use handlegraph::hashgraph::Path;
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use std::sync::{Arc, Mutex};

use bit_vec::BitVec;
use handlegraph::{handlegraph::HandleGraph, hashgraph::HashGraph};

use priority_queue::DoublePriorityQueue;

use crate::{coordinate::Coordinate, utility};

pub fn extract(
    graph: &HashGraph,
    k_len: usize,
    w_len: usize,
    n: usize,
) -> FxHashMap<Vec<String>, FxHashMap<(Coordinate, Coordinate), BitVec>> {
    // extract paths from graph
    let paths = graph.paths.values().collect::<Vec<&Path>>();
    let paths_number = paths.len();

    // init FxHashMaps to store result
    let strobemers = Arc::new(Mutex::new(FxHashMap::<
        Vec<String>,
        FxHashMap<(Coordinate, Coordinate), BitVec>, // OSS: coordinate solo del primo kmer, come scritto in pubblicazione
                                                     // cercare di capire come fare a salvare coordinate di tutti i kmer
    >::default()));

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

        // init hash heap for each window
        let mut hash_heaps = Vec::new();
        for i in 0..n - 1 {
            hash_heaps.push(DoublePriorityQueue::with_capacity(w_len - k_len + 1));
            for j in k_len + w_len * i..k_len + w_len * (i + 1) {
                hash_heaps[i].push(j, hash(&path_seq[j..j + k_len]));
            }
        }

        for i in 0..path_seq.len() - (k_len + w_len * (n - 1)) + 1 {
            let mut strobemer = vec![String::from(&path_seq[i..i + k_len])];

            for win in 0..n - 1 {
                let (minimizer_pos, __) = hash_heaps[win].peek_min().unwrap();
                let min_kmer = String::from(&path_seq[*minimizer_pos..*minimizer_pos + k_len]);

                strobemer.push(min_kmer);
            }

            let kmer_start = Coordinate::build(path_nodes_id[i], i, &path_nodes_id);

            let kmer_end =
                Coordinate::build(path_nodes_id[i + k_len - 1], i + k_len - 1, &path_nodes_id);

            let mut strobemers_locked = strobemers.lock().unwrap();
            utility::update_strobemers(
                &mut strobemers_locked,
                strobemer,
                kmer_start,
                kmer_end,
                path,
                paths_number,
            );

            for win in 0..n - 1 {
                let kmer_to_add =
                    &path_seq[i + k_len + w_len * win..i + k_len + w_len * win + k_len];
                utility::update_hash_heap(
                    &mut hash_heaps[win],
                    kmer_to_add,
                    i + (k_len + w_len * (win - 1)),
                    i + (k_len + w_len * win),
                );
            }
        }
    });
    Arc::try_unwrap(strobemers).unwrap().into_inner().unwrap()
}
