use anyhow::Result;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Genesis {
    date: Timestamp,
    /// The unique identifier for this running instance of the blockchain.
    chain_id: u16,
    /// The maximum number of transactions allowed in each block.
    transections_per_block: u16,
    /// The difficulty level for mining new blocks.
    difficulty: u16,
    /// The reward given to miners for successfully mining a block.
    mining_reward: u64,
    /// The price of gas for executing transactions.
    gas_price: u64,
    /// A mapping of account addresses to their initial balances.
    pub(crate) balances: HashMap<String, u64>,
}

impl Genesis {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Genesis> {
        let content = std::fs::read_to_string(path)?;
        let genesis: Genesis = serde_json::from_str(&content)?;
        Ok(genesis)
    }
}

#[cfg(test)]
mod tests {
    use crate::genesis::Genesis;

    #[test]
    fn load_genesis() {
        let genesis = Genesis::load("../../assets/genesis.json").unwrap();
        insta::with_settings!({sort_maps => true}, {
                insta::assert_json_snapshot!(genesis);
        });
    }
}
