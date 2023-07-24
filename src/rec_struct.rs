use bit_vec::BitVec;

pub struct RecStruct {
    pub first_start: usize,
    pub first_end: usize,
    pub first_paths: BitVec,
    pub first_read_start: usize,
    pub second_start: usize,
    pub second_end: usize,
    pub second_paths: BitVec,
    pub second_read_start: usize,
}
impl RecStruct {
    pub fn new() -> RecStruct {
        RecStruct {
            first_start: 0,
            first_end: 0,
            first_paths: BitVec::new(),
            first_read_start: 0,
            second_start: 0,
            second_end: 0,
            second_paths: BitVec::new(),
            second_read_start: 0,
        }
    }

    pub fn build_rec_struct(
        first_start: usize,
        first_end: usize,
        first_paths: BitVec,
        first_read_start: usize,
        second_start: usize,
        second_end: usize,
        second_paths: BitVec,
        second_read_start: usize,
    ) -> RecStruct {
        RecStruct {
            first_start,
            first_end,
            first_paths,
            first_read_start,
            second_start,
            second_end,
            second_paths,
            second_read_start,
        }
    }
}

impl Default for RecStruct {
    fn default() -> Self {
        Self::new()
    }
}
