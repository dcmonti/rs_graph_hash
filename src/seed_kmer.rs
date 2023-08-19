use bit_vec::BitVec;

use crate::coordinate::Coordinate;


#[derive(Debug)]
pub struct SeedKmer {
    pub positions: [Coordinate; 4], // [k-mer start, k-mer end, read start, read end]
    pub paths: BitVec,
}
impl SeedKmer {
    pub fn new() -> SeedKmer {
        SeedKmer {
            positions: [Coordinate::empty_new(); 4],
            paths: BitVec::new(),
        }
    }

    pub fn build(
        start: Coordinate,
        end: Coordinate,
        paths: BitVec,
        read_start: Coordinate,
        read_end: Coordinate,
    ) -> SeedKmer {
        SeedKmer {
            positions: [start, end, read_start, read_end],
            paths,
        }
    }

    pub fn update_ends(&mut self, end: Coordinate, read_end: Coordinate, paths: BitVec) {
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
