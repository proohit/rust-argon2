use crate::block::Block;

pub struct Memory {
    rows: usize,

    cols: usize,

    blocks: Box<[Block]>,
}

impl Memory {
    pub fn new(lanes: u32, lane_length: u32) -> Memory {
        let rows = lanes as usize;
        let cols = lane_length as usize;
        let total = rows * cols;
        let blocks = vec![Block::zero(); total].into_boxed_slice();
        Memory { rows, cols, blocks }
    }

    #[cfg(feature = "crossbeam-utils")]
    pub fn as_lanes_mut(&mut self) -> Vec<&mut Memory> {
        let ptr: *mut Memory = self;
        let mut vec = Vec::with_capacity(self.rows);
        for _ in 0..self.rows {
            vec.push(unsafe { &mut (*ptr) });
        }
        vec
    }
}
