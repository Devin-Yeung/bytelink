use crate::mempool::Key;
use crate::mempool::selector::{Selector, TxInfo};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct TimebasedSelector {}

/// First-Come-First-Serve selector implementation base on the timestamp
impl TimebasedSelector {
    pub fn new() -> Self {
        TimebasedSelector {}
    }
}

#[derive(Eq)]
struct Item {
    timestamp: u64,
    key: Key,
}

impl PartialEq<Self> for Item {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl PartialOrd<Self> for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.timestamp.partial_cmp(&self.timestamp)
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        other.timestamp.cmp(&self.timestamp)
    }
}

impl Selector for TimebasedSelector {
    type Key = Key;
    fn select<I, T>(&self, pool: I, max_block_size: usize) -> Vec<Self::Key>
    where
        T: TxInfo,
        I: Iterator<Item = T>,
    {
        let mut heap: BinaryHeap<Item> = BinaryHeap::with_capacity(max_block_size);
        for tx in pool {
            let item = Item {
                timestamp: tx.timestamp(),
                key: tx.key(),
            };
            heap.push(item);
        }
        let mut selected_keys = Vec::with_capacity(max_block_size);
        for _ in 0..max_block_size {
            if let Some(item) = heap.pop() {
                selected_keys.push(item.key);
            } else {
                break;
            }
        }
        selected_keys
    }
}

#[cfg(test)]
mod tests {
    use super::TimebasedSelector;
    use crate::mempool::Key;
    use crate::mempool::selector::{Selector, TxInfo};
    use bytelink_crypto::address::Address;
    use rand::{Rng, rng};

    pub struct MockTxInfo {
        pub timestamp: u64,
    }

    impl MockTxInfo {
        pub fn new(timestamp: u64) -> Self {
            MockTxInfo { timestamp }
        }
    }

    impl TxInfo for MockTxInfo {
        fn timestamp(&self) -> u64 {
            self.timestamp
        }

        fn key(&self) -> Key {
            // encode the timestamp into the nonce so tests can assert ordering
            Key::new(Address::random(rng()), self.timestamp)
        }
    }

    #[test]
    fn select_orders_by_earliest_timestamp() {
        // create transactions with different timestamps (unordered)
        let txs = vec![
            MockTxInfo::new(300),
            MockTxInfo::new(100),
            MockTxInfo::new(200),
        ];

        let selector = TimebasedSelector::new();
        let selected = selector.select(txs.into_iter(), 10);

        // resulting keys' nonces should be in ascending order (100, 200, 300)
        let nonces: Vec<u64> = selected.into_iter().map(|k| k.nonce).collect();
        assert_eq!(nonces, vec![100, 200, 300]);
    }

    #[test]
    fn select_respects_max_block_size() {
        let txs = (1..=5).map(|i| MockTxInfo::new(i)).collect::<Vec<_>>();
        let selector = TimebasedSelector::new();
        let selected = selector.select(txs.into_iter(), 3);
        assert_eq!(selected.len(), 3);
    }

    #[test]
    fn select_empty_pool_returns_empty() {
        let txs: Vec<MockTxInfo> = Vec::new();
        let selector = TimebasedSelector::new();
        let selected = selector.select(txs.into_iter(), 10);
        assert!(selected.is_empty());
    }
}
