use std::cmp::Ordering;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct Coordinate {
    pub node_id: usize,
    pub offset: usize,
}

impl Coordinate {
    pub fn empty_new() -> Coordinate {
        Coordinate {
            node_id: 0,
            offset: 0,
        }
    }
    pub fn new(position: usize) -> Coordinate {
        Coordinate {
            node_id: 0,
            offset: position,
        }
    }
    pub fn build(handle_id: u64, position: usize, handles_pos: &[u64]) -> Coordinate {
        let mut offset = 0;
        let mut start = position;
        while start > 0 && handles_pos[start - 1] == handle_id {
            start -= 1;
            offset += 1;
        }

        Coordinate {
            node_id: handle_id as usize,
            offset,
        }
    }

    pub fn equal(&self, other: &Coordinate) -> bool {
        self.node_id == other.node_id && self.offset == other.offset
    }

    pub fn included(&self, start_coor: &Coordinate, end_coor: &Coordinate) -> bool {
        let after_start = match self.node_id.cmp(&start_coor.node_id) {
            Ordering::Greater => true,
            Ordering::Equal => self.offset >= start_coor.offset,
            Ordering::Less => false,
        };

        let before_end = match self.node_id.cmp(&end_coor.node_id) {
            Ordering::Greater => false,
            Ordering::Equal => self.offset <= end_coor.offset,
            Ordering::Less => true,
        };

        after_start && before_end
    }
}
