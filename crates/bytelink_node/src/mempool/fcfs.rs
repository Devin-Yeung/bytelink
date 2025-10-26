use crate::mempool::Key;
use crate::mempool::selector::{Selector, TxInfo};

pub struct TimebasedSelector {}

/// First-Come-First-Serve selector implementation base on the timestamp
impl TimebasedSelector {
    pub fn new() -> Self {
        TimebasedSelector {}
    }
}

impl Selector for TimebasedSelector {
    fn select<I, T>(&self, pool: I, max_block_size: usize) -> Vec<Key>
    where
        T: TxInfo,
        I: Iterator<Item = T>,
    {
        // use a heap to select transactions based on timestamp
        todo!()
    }
}
