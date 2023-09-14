use rustc_hash::FxHashMap as HashMap;

use bit_vec::BitVec;

use syntect::dumps::{dump_to_file, from_dump_file};

use crate::coordinate::Coordinate;

pub fn dump_unique_kmers(
    unique_kmers: &HashMap<String, (Coordinate, Coordinate, BitVec)>,
    k: usize,
    path: &String,
) {
    dump_to_file(&(unique_kmers, k), path).unwrap();
}

pub fn load_unique_kmers(
    path: &String,
) -> (HashMap<String, (Coordinate, Coordinate, BitVec)>, usize) {
    let (unique_kmers, k): (HashMap<String, (Coordinate, Coordinate, BitVec)>, usize) =
        from_dump_file(path).unwrap();
    (unique_kmers, k)
}
