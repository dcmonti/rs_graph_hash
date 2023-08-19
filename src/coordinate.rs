
#[derive(Clone, Debug, Copy)]
pub struct Coordinate {
    pub node_id: usize, // [k-mer start, k-mer end, read start, read end]
    pub offset: usize,
}

impl Coordinate {
    pub fn empty_new() -> Coordinate {
        Coordinate { node_id: 0, offset: 0 }
    }
    pub fn new(position: usize) -> Coordinate {
        Coordinate { node_id: 0, offset: position }
    }
    pub fn build(
        handle_id: u64,
        position: usize,
        handles_pos: &Vec<u64>
    ) -> Coordinate {
        let mut offset = 0;
        let mut start = position;
        while start > 0 && handles_pos[start - 1] == handle_id {
            start -= 1;
            offset += 1;
        }
        
        Coordinate { node_id: handle_id as usize, offset: offset }
    }

    pub fn equal(self, other: &Coordinate) -> bool {
        self.node_id == other.node_id && self.offset == other.offset
    }
}

