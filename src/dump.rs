use std::collections::HashMap;

use bit_vec::BitVec;

use syntect::dumps::{dump_to_file, from_dump_file};

use crate::coordinate::Coordinate;

pub fn dump_unique_kmers(
    unique_kmers: &HashMap<String, (Coordinate, Coordinate, BitVec)>,
    k: usize,
    path: &String,
) {
    dump_to_file(&(convert_to_serializable_format(unique_kmers), k), path).unwrap();
}

pub fn load_unique_kmers(
    path: &String,
) -> (HashMap<String, (Coordinate, Coordinate, BitVec)>, usize) {
    let (tmp_hash, k): (HashMap<String, (Coordinate, Coordinate, Vec<u8>)>, usize) =
        from_dump_file(path).unwrap();
    let unique_kmers = convert_to_bitvec(&tmp_hash);
    (unique_kmers, k)
}
fn convert_to_serializable_format(
    unique_kmers: &HashMap<String, (Coordinate, Coordinate, BitVec)>,
) -> HashMap<std::string::String, (Coordinate, Coordinate, Vec<u8>)> {
    let mut new_unique_kmers = HashMap::new();
    for (kmer, (start, end, paths)) in unique_kmers.iter() {
        let vec_paths = paths.to_bytes();
        new_unique_kmers.insert(kmer.to_owned(), (*start, *end, vec_paths));
    }
    new_unique_kmers
}

fn convert_to_bitvec(
    hashmap: &HashMap<String, (Coordinate, Coordinate, Vec<u8>)>,
) -> HashMap<std::string::String, (Coordinate, Coordinate, BitVec)> {
    let mut new_unique_kmers = HashMap::new();
    for (kmer, (start, end, paths)) in hashmap.iter() {
        let bitvec_paths = BitVec::from_bytes(paths);
        new_unique_kmers.insert(kmer.to_owned(), (*start, *end, bitvec_paths));
    }
    new_unique_kmers
}
