use std::collections::{HashMap, HashSet};

use bit_vec::BitVec;

use crate::rec_struct::RecStruct;

pub fn find_recomb_kmers(
    read: &String,
    unique_kmers: &HashMap<String, ((usize, usize), BitVec)>,
    k: usize,
    rec_mode: i32,
) -> Vec<RecStruct> {
    let kmers = filter_read_kmers(read, unique_kmers, k, rec_mode);
    let mut recombs: Vec<RecStruct> = Vec::new();
    for (i, first_kmer) in kmers.iter().enumerate() {
        let mut i_paths = BitVec::from_elem(first_kmer.paths.len(), true);
        i_paths.and(&first_kmer.paths);

        for second_kmer in kmers.iter().skip(i) {
            let mut common_paths = BitVec::from_elem(second_kmer.paths.len(), true);
            if rec_mode == 0 || rec_mode == 2 {
                common_paths.and(&second_kmer.paths);
                common_paths.and(&i_paths);
            } else if rec_mode == 1 {
                // consider only consecutive positions
                // TODO: some fixing needed
                if first_kmer.read_start + k == second_kmer.read_start {
                    common_paths.and(&second_kmer.paths);
                    common_paths.and(&i_paths);
                }
            }

            if !common_paths.any() {
                let rec = RecStruct::build_rec_struct(
                    first_kmer.start,
                    first_kmer.end,
                    first_kmer.paths.clone(),
                    second_kmer.start,
                    second_kmer.end,
                    second_kmer.paths.clone(),
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
) -> Vec<CandidateKmer> {
    let mut candidate_kmers = Vec::new();
    let mut found_kmers = HashSet::new();

    for i in 0..read.len() - k + 1 {
        let read_kmer: String = read.chars().skip(i).take(k).collect();
        if unique_kmers.contains_key(&read_kmer) && !found_kmers.contains(&read_kmer) {
            let ((start, end), paths)= unique_kmers.get(&read_kmer).unwrap().to_owned();
            let candidate_kmer = CandidateKmer::build(start, end, paths, i);
            if rec_mode == 2 {
                // if k-mers positions already covered by previous ones, skip
                if candidate_kmers.is_empty() {
                    candidate_kmers.push(candidate_kmer);
                    found_kmers.insert(read_kmer);
                } else {
                    let last_kmer = candidate_kmers.last().unwrap();
                    if !(start > last_kmer.start
                        && start < last_kmer.end)
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

pub struct CandidateKmer {
    pub start: usize,
    pub end: usize,
    pub paths: BitVec,
    pub read_start: usize
}
impl CandidateKmer {
    pub fn new() -> CandidateKmer {
        CandidateKmer {
            start: 0,
            end: 0,
            paths: BitVec::new(),
            read_start: 0
        }
    }

    pub fn build(
        start: usize,
        end: usize,
        paths: BitVec,
        read_start: usize
    ) -> CandidateKmer {
        CandidateKmer {
            start,
            end,
            paths,
            read_start
        }
    }
}

impl Default for CandidateKmer {
    fn default() -> Self {
        Self::new()
    }
}
