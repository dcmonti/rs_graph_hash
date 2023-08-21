use std::collections::HashMap;

use bit_vec::BitVec;

use crate::{coordinate::Coordinate, seed_kmer::SeedKmer};

pub fn match_read_kmers(
    read: &String,
    unique_kmers: &HashMap<String, (Coordinate, Coordinate, BitVec)>,
    k: usize,
) -> Vec<SeedKmer> {
    let mut read_to_path_align = Vec::new();

    let mut i = 0;
    while i < read.len() - k + 1 {
        let read_kmer: String = read.chars().skip(i).take(k).collect();
        if unique_kmers.contains_key(&read_kmer) {
            let (start, end, paths) = unique_kmers.get(&read_kmer).unwrap().to_owned();
            if read_to_path_align.is_empty() {
                let seed_kmer = SeedKmer::build(
                    start,
                    end,
                    paths,
                    Coordinate::new(i),
                    Coordinate::new(i + k - 1),
                );
                read_to_path_align.push(seed_kmer)
            } else {
                let last_match = read_to_path_align.last_mut().unwrap();

                let mut actual_paths = last_match.paths.clone();
                actual_paths.and(&paths);

                if actual_paths.any()
                    && start.included(&last_match.positions[0], &last_match.positions[1])
                {
                    last_match.update_ends(end, Coordinate::new(i + k - 1), actual_paths)
                } else {
                    let seed_kmer = SeedKmer::build(
                        start,
                        end,
                        paths,
                        Coordinate::new(i),
                        Coordinate::new(i + k - 1),
                    );
                    read_to_path_align.push(seed_kmer)
                }
            }

            i += k / 2
        } else {
            i += 1
        }
    }
    read_to_path_align
}