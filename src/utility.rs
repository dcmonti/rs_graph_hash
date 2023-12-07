use std::{collections::HashMap, hash::BuildHasherDefault, sync::MutexGuard};

use bit_vec::BitVec;
use fxhash::hash;
use handlegraph::hashgraph::Path;
use priority_queue::DoublePriorityQueue;
use rustc_hash::FxHashMap;

use crate::coordinate::Coordinate;

pub fn update_hash_heap(
    hash_heap: &mut DoublePriorityQueue<usize, usize>,
    k_mer: &str,
    to_remove: usize,
    i: usize,
) {
    hash_heap.remove(&to_remove);
    hash_heap.push(i, hash(k_mer));
}

pub fn update_minimizers(
    minimizers_locked: &mut MutexGuard<
        '_,
        HashMap<
            std::string::String,
            HashMap<
                (Coordinate, Coordinate),
                bit_vec::BitVec,
                BuildHasherDefault<rustc_hash::FxHasher>,
            >,
            BuildHasherDefault<rustc_hash::FxHasher>,
        >,
    >,
    kmer: &str,
    kmer_start: Coordinate,
    kmer_end: Coordinate,
    path: &Path,
    paths_number: usize,
) {
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
        let mut kmer_hashmap = FxHashMap::<(Coordinate, Coordinate), BitVec>::default();
        kmer_hashmap.insert((kmer_start, kmer_end), kmer_paths);
        minimizers_locked.insert(kmer.to_string(), kmer_hashmap);
    }
}

pub fn update_strobemers(
    minimizers_locked: &mut MutexGuard<
        '_,
        HashMap<
            Vec<String>,
            HashMap<
                (Coordinate, Coordinate),
                bit_vec::BitVec,
                BuildHasherDefault<rustc_hash::FxHasher>,
            >,
            BuildHasherDefault<rustc_hash::FxHasher>,
        >,
    >,
    strobemer: Vec<String>,
    kmer_start: Coordinate,
    kmer_end: Coordinate,
    path: &Path,
    paths_number: usize,
) {
    if minimizers_locked.contains_key(&strobemer) {
        let old_kmer_hashmap = minimizers_locked.get_mut(&strobemer).unwrap();

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
        let mut kmer_hashmap = FxHashMap::<(Coordinate, Coordinate), BitVec>::default();
        kmer_hashmap.insert((kmer_start, kmer_end), kmer_paths);
        minimizers_locked.insert(strobemer, kmer_hashmap);
    }
}
