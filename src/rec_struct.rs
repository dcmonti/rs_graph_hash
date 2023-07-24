use bit_vec::BitVec;

pub struct RecStruct {
    pub first_start: usize,
    pub first_end: usize,
    pub first_paths: BitVec,
    pub second_start: usize,
    pub second_end: usize,
    pub second_paths: BitVec,
}
impl RecStruct {
    pub fn new() -> RecStruct {
        RecStruct {
            first_start: 0,
            first_end: 0,
            first_paths: BitVec::new(),
            second_start: 0,
            second_end: 0,
            second_paths: BitVec::new(),
        }
    }

    pub fn build_rec_struct(
        first_start: usize,
        first_end: usize,
        first_paths: BitVec,
        second_start: usize,
        second_end: usize,
        second_paths: BitVec,
    ) -> RecStruct {
        RecStruct {
            first_start,
            first_end,
            first_paths,
            second_start,
            second_end,
            second_paths,
        }
    }
}

impl Default for RecStruct {
    fn default() -> Self {
        Self::new()
    }
}