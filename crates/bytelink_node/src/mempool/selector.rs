use crate::database::transaction::BlockTx;
use crate::mempool::Key;
use dashmap::mapref::multiple::RefMulti;

/// A selector is responsible for selecting transactions from the mempool
pub trait Selector {
    /// Select transactions from the mempool up to max_txs
    fn select<I, T>(&self, pool: I, max_block_size: usize) -> Vec<Key>
    where
        T: TxInfo,
        I: Iterator<Item = T>;
}

pub trait TxInfo {
    fn timestamp(&self) -> u64;
}

impl TxInfo for RefMulti<'_, Key, BlockTx> {
    fn timestamp(&self) -> u64 {
        todo!()
    }
}
