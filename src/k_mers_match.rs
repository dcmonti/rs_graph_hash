use std::collections::{HashMap, HashSet};

use bit_vec::BitVec;

use crate::{path_graph::PathGraph, rec_struct::RecStruct};

pub fn find_recomb_kmers(
    read: &String,
    unique_kmers: &HashMap<String, ((usize, usize), BitVec)>,
    k: usize,
    rec_mode: i32,
    graph: &PathGraph,
) -> Vec<RecStruct> {
    let kmers = filter_read_kmers(read, unique_kmers, k, rec_mode);
    let mut recombs: Vec<RecStruct> = Vec::new();
    for (i, ((i_start, i_end), kmer_paths)) in kmers.iter().enumerate() {
        let mut i_paths = BitVec::from_elem(kmer_paths.len(), true);
        i_paths.and(kmer_paths);

        for ((j_start, j_end), j_paths) in kmers.iter().skip(i) {
            let mut common_paths = BitVec::from_elem(j_paths.len(), true);
            if rec_mode == 0 || rec_mode == 2 {
                common_paths.and(j_paths);
                common_paths.and(&i_paths);
            } else if rec_mode == 1 {
                // consider only consecutive positions
                // TODO: some fixing needed
                if graph.nws[*i_end] {
                    let succs = graph.succ_hash.get_succs(*i_end);
                    if succs.contains(j_start) {
                        common_paths.and(j_paths);
                        common_paths.and(&i_paths);
                    }
                } else if *j_start == i_end + 1 {
                    common_paths.and(j_paths);
                    common_paths.and(&i_paths);
                }
            }

            if !common_paths.any() {
                let rec = RecStruct::build_rec_struct(
                    *i_start,
                    *i_end,
                    i_paths.clone(),
                    *j_start,
                    *j_end,
                    j_paths.clone(),
                );
                recombs.push(rec);
                //break;
            }
        }
    }

    recombs
}

fn filter_read_kmers(
    read: &String,
    unique_kmers: &HashMap<String, ((usize, usize), BitVec)>,
    k: usize,
    rec_mode: i32,
) -> Vec<((usize, usize), BitVec)> {
    let mut candidate_kmers = Vec::new();
    let mut found_kmers = HashSet::new();

    for i in 0..read.len() - k + 1 {
        let read_kmer: String = read.chars().skip(i).take(k).collect();
        if unique_kmers.contains_key(&read_kmer) && !found_kmers.contains(&read_kmer) {
            let candidate_kmer = unique_kmers.get(&read_kmer).unwrap().to_owned();
            if rec_mode == 2 {
                // if k-mers positions already covered by previous ones, skip
                if candidate_kmers.is_empty() {
                    candidate_kmers.push(candidate_kmer);
                    found_kmers.insert(read_kmer);
                } else {
                    let ((last_kmer_start, last_kmer_end), _) = candidate_kmers.last().unwrap();
                    if !(candidate_kmer.0 .0 > *last_kmer_start
                        && candidate_kmer.0 .0 < *last_kmer_end)
                    {
                        candidate_kmers.push(candidate_kmer);
                        found_kmers.insert(read_kmer);
                    }
                }
            } else {
                candidate_kmers.push(candidate_kmer);
                found_kmers.insert(read_kmer);
            }
        }
    }
    candidate_kmers
}
