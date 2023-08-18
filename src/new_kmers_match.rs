use std::collections::HashMap;

use bit_vec::BitVec;

pub fn match_read_kmers(
    read: &String,
    unique_kmers: &HashMap<String, ((usize, usize), BitVec)>,
    k: usize,
) -> Vec<SeedKmer> {
    let mut read_to_path_align = Vec::new();

    let mut i = 0;
    while i < read.len() - k + 1 {
        let read_kmer: String = read.chars().skip(i).take(k).collect();
        if unique_kmers.contains_key(&read_kmer) {
            let ((start, end), paths) = unique_kmers.get(&read_kmer).unwrap().to_owned();
            if read_to_path_align.is_empty() {
                let seed_kmer = SeedKmer::build(start, end, paths, i, i + k - 1);
                read_to_path_align.push(seed_kmer)
            } else {
                let last_match = read_to_path_align.last_mut().unwrap();

                let mut actual_paths = last_match.paths.clone();
                actual_paths.and(&paths);

                if actual_paths.any() {
                    last_match.update_ends(end, i + k - 1, actual_paths)
                } else {
                    let seed_kmer = SeedKmer::build(start, end, paths, i, i + k - 1);
                    read_to_path_align.push(seed_kmer)
                }
            }

            i += k
        } else {
            i += 1
        }
    }
    read_to_path_align
}

#[derive(Debug)]
pub struct SeedKmer {
    pub positions: [usize; 4], // [k-mer start, k-mer end, read start, read end]
    pub paths: BitVec,
}
impl SeedKmer {
    pub fn new() -> SeedKmer {
        SeedKmer {
            positions: [0; 4],
            paths: BitVec::new(),
        }
    }

    pub fn build(
        start: usize,
        end: usize,
        paths: BitVec,
        read_start: usize,
        read_end: usize,
    ) -> SeedKmer {
        SeedKmer {
            positions: [start, end, read_start, read_end],
            paths,
        }
    }

    pub fn update_ends(&mut self, end: usize, read_end: usize, paths: BitVec) {
        self.positions[1] = end;
        self.positions[3] = read_end;
        self.paths = paths
    }
}

impl Default for SeedKmer {
    fn default() -> Self {
        Self::new()
    }
}
