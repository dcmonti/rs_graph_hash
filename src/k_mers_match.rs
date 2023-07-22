use std::collections::{HashMap, HashSet};

use bit_vec::BitVec;

use crate::rec_struct::RecStruct;

pub fn find_recomb_kmers(
    read: &String,
    unique_kmers: &HashMap<String, (usize, BitVec)>,
    k: usize,
) -> Vec<RecStruct> {
    let kmers = filter_read_kmers(read, unique_kmers, k);
    let mut recombs: Vec<RecStruct> = Vec::new();
    for (i, (i_start, kmer_paths)) in kmers.iter().enumerate() {
        let mut i_paths = BitVec::from_elem(kmer_paths.len(), true);
        i_paths.and(kmer_paths);

        for (j_start, j_paths) in kmers.iter().skip(i) {
            let mut common_paths = BitVec::from_elem(j_paths.len(), true);
            common_paths.and(j_paths);
            common_paths.and(&i_paths);

            if !common_paths.any() {
                let rec = RecStruct::build_rec_struct(
                    *i_start,
                    i_paths.clone(),
                    *j_start,
                    j_paths.clone(),
                );
                recombs.push(rec)
            }
        }
    }

    recombs
}

fn filter_read_kmers(
    read: &String,
    unique_kmers: &HashMap<String, (usize, BitVec)>,
    k: usize,
) -> Vec<(usize, BitVec)> {
    let mut candidate_kmers = Vec::new();
    let mut found_kmers = HashSet::new();

    for i in 0..read.len() - k + 1 {
        let read_kmer: String = read.chars().skip(i).take(k).collect();
        if unique_kmers.contains_key(&read_kmer) && !found_kmers.contains(&read_kmer) {
            candidate_kmers.push(unique_kmers.get(&read_kmer).unwrap().to_owned());
            found_kmers.insert(read_kmer);
        }
    }
    candidate_kmers
}
