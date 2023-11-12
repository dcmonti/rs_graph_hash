use handlegraph::hashgraph::Path;
use rayon::prelude::*;
use rustc_hash::FxHashMap as FxHashMap;
use fxhash::hash;
use std::{sync::{Arc, Mutex, MutexGuard}, collections::HashMap, hash::BuildHasherDefault};

use bit_vec::BitVec;
use handlegraph::{handlegraph::HandleGraph, hashgraph::HashGraph};

use priority_queue::DoublePriorityQueue;

use crate::coordinate::Coordinate;

pub fn extract(
    graph: &HashGraph,
    k_len: usize,
    w_len: usize,
) -> FxHashMap::<
String,
FxHashMap::<
            (Coordinate, Coordinate),
            BitVec>
>{
    // extract paths from graph
    let paths = graph.paths.values().collect::<Vec<&Path>>();
    let paths_number = paths.len();

    // init FxHashMaps to store result
    let minimizers = Arc::new(Mutex::new(FxHashMap::<
        String,
        FxHashMap::<
            (Coordinate, Coordinate),
            BitVec>
    >::default()));
    

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

        let mut hash_heap = DoublePriorityQueue::with_capacity(w_len-k_len+1); 
        for i in 0..w_len - k_len + 1{
            hash_heap.push(i, hash(&path_seq[i..i + k_len]));
        }
        
        for i in w_len - k_len + 1..path_seq.len() - k_len + 1 {
            let (minimizer_pos, __) = hash_heap.peek_min().unwrap();
            let min_kmer = &path_seq[*minimizer_pos..*minimizer_pos + k_len];

            let kmer_start = Coordinate::build(path_nodes_id[*minimizer_pos], *minimizer_pos, &path_nodes_id);
            let kmer_end =
                Coordinate::build(path_nodes_id[minimizer_pos + k_len - 1], minimizer_pos + k_len - 1, &path_nodes_id);

            let mut minimizers_locked = minimizers.lock().unwrap();
            update_minimizers(&mut minimizers_locked, min_kmer, kmer_start, kmer_end, path, paths_number);  

            let kmer_to_add = &path_seq[i..i+k_len];
            update_hash_heap(&mut hash_heap, kmer_to_add, k_len, i);        
        }
    });

    Arc::try_unwrap(minimizers).unwrap().into_inner().unwrap()
}

fn update_hash_heap(hash_heap: &mut DoublePriorityQueue<usize, usize>, k_mer: &str, k_len: usize, i: usize) {
    hash_heap.remove(&(i-k_len));
    hash_heap.push(i, hash(k_mer));
}

fn update_minimizers(
    minimizers_locked: &mut MutexGuard<'_, HashMap<std::string::String, HashMap<(Coordinate, Coordinate), bit_vec::BitVec, BuildHasherDefault<rustc_hash::FxHasher>>, BuildHasherDefault<rustc_hash::FxHasher>>>, 
    kmer: &str, 
    kmer_start: Coordinate, 
    kmer_end: Coordinate,
    path: &Path,
    paths_number: usize) {
    if minimizers_locked.contains_key(kmer) {
        let old_kmer_hashmap = minimizers_locked.get_mut(kmer).unwrap();
        if old_kmer_hashmap.contains_key(&(kmer_start, kmer_end)) {
            
            let old_paths = old_kmer_hashmap.get_mut(&(kmer_start, kmer_end)).unwrap();
            old_paths.set(path.path_id as usize, true);
        } else {
            let mut kmer_paths = BitVec::from_elem(paths_number, false);
            kmer_paths.set(path.path_id as usize, true);
            old_kmer_hashmap.insert((kmer_start, kmer_end), kmer_paths);
        }
        
    } else {
        let mut kmer_paths = BitVec::from_elem(paths_number, false);
        kmer_paths.set(path.path_id as usize, true);
        let mut kmer_hashmap = FxHashMap::<
            (Coordinate, Coordinate),
            BitVec>::default();
        kmer_hashmap.insert((kmer_start, kmer_end), kmer_paths);
        minimizers_locked.insert(kmer.to_string(), kmer_hashmap);
    }
}