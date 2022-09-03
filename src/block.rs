use crate::common;

pub struct Block([u64; common::QWORDS_IN_BLOCK]);

impl Block {
    pub fn zero() -> Block {
        Block([0u64; common::QWORDS_IN_BLOCK])
    }
}

impl Clone for Block {
    fn clone(&self) -> Block {
        Block(self.0)
    }
}
