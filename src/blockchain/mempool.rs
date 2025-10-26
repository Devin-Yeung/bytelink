mod fcfs;
mod selector;

use crate::blockchain::database::account::Address;
use crate::blockchain::database::transaction::BlockTx;
use crate::blockchain::mempool::fcfs::TimebasedSelector;
use crate::blockchain::mempool::selector::{Selector, TxInfo};
use anyhow::Result;
use dashmap::DashMap;

/// A mempool is staging area for unconfirmed transactions in a blockchain.
pub struct MemPool {
    pool: DashMap<Key, BlockTx>,
}

#[derive(PartialEq, Eq, Hash)]
pub struct Key {
    sender: Address,
    nonce: u64,
}

impl Key {
    pub fn new(sender: Address, nonce: u64) -> Self {
        Key { sender, nonce }
    }
}

impl MemPool {
    /// create a new empty mempool.
    pub fn new() -> Self {
        MemPool {
            pool: DashMap::new(),
        }
    }

    /// number of transactions in the mempool.
    pub fn len(&self) -> usize {
        self.pool.len()
    }

    pub fn upsert(&self, tx: BlockTx) -> Result<()> {
        // use address with nonce as the key
        let key = Key::new(tx.tx().from.clone(), tx.tx().nonce);

        if let Some(entry) = self.pool.get(&key) {
            // ethereum require 10% bump in the tip to replace an existing tx
            let original_tip = entry.tx().tip;
            if tx.tx().tip < original_tip + original_tip / 10 {
                return Err(anyhow::anyhow!(
                    "transaction tip too low to replace existing transaction"
                ));
            }
        }

        self.pool.insert(key, tx);
        Ok(())
    }

    pub fn delete(&self, key: &Key) {
        self.pool.remove(key);
    }

    pub fn truncate(&self) {
        self.pool.clear();
    }
}
