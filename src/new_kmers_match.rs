use std::collections::HashMap;

use bit_vec::BitVec;

use crate::rec_struct::RecStruct;

pub fn match_read_kmers(
    read: &String,
    unique_kmers: &HashMap<String, ((usize, usize), BitVec)>,
    k: usize,
) -> Vec<RecStruct> {
    let mut read_to_path_align = Vec::new();

    let mut i = 0;
    while i < read.len() - k + 1 {
        let read_kmer: String = read.chars().skip(i).take(k).collect();
        if unique_kmers.contains_key(&read_kmer) {
            let ((start, end), paths) = unique_kmers.get(&read_kmer).unwrap().to_owned();
            if read_to_path_align.is_empty() {
                let candidate_kmer = CandidateKmer::build(start, end, paths, i, i + k - 1);
                read_to_path_align.push(candidate_kmer)
            } else {
                let last_match = read_to_path_align.pop().unwrap();

                let mut actual_paths = last_match.paths.clone();
                actual_paths.and(&paths);

                if actual_paths.any() {
                    let candidate_kmer = CandidateKmer::build(
                        last_match.start,
                        end,
                        actual_paths,
                        last_match.read_start,
                        i + k - 1,
                    );
                    read_to_path_align.push(candidate_kmer)
                } else {
                    let candidate_kmer = CandidateKmer::build(start, end, paths, i, i + k - 1);
                    read_to_path_align.push(last_match);
                    read_to_path_align.push(candidate_kmer)
                }
            }

            i += k
        } else {
            i += 1
        }
    }

    let mut recombs = Vec::new();
    for i in 0..read_to_path_align.len() - 1 {
        let first_kmer = &read_to_path_align[i];
        let second_kmer = &read_to_path_align[i + 1];
        let rec = RecStruct::build_rec_struct(
            first_kmer.start,
            first_kmer.end,
            first_kmer.paths.to_owned(),
            first_kmer.read_start,
            second_kmer.start,
            second_kmer.end,
            second_kmer.paths.to_owned(),
            second_kmer.read_start,
        );
        recombs.push(rec)
    }
    recombs
}

#[derive(Debug)]
pub struct CandidateKmer {
    pub start: usize,
    pub end: usize,
    pub paths: BitVec,
    pub read_start: usize,
    pub read_end: usize,
}
impl CandidateKmer {
    pub fn new() -> CandidateKmer {
        CandidateKmer {
            start: 0,
            end: 0,
            paths: BitVec::new(),
            read_start: 0,
            read_end: 0,
        }
    }

    pub fn build(
        start: usize,
        end: usize,
        paths: BitVec,
        read_start: usize,
        read_end: usize,
    ) -> CandidateKmer {
        CandidateKmer {
            start,
            end,
            paths,
            read_start,
            read_end,
        }
    }
}

impl Default for CandidateKmer {
    fn default() -> Self {
        Self::new()
    }
}
