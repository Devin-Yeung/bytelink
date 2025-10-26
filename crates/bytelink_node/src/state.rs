use crate::database::Database;
use crate::genesis::Genesis;
use anyhow::Result;
use bytelink_crypto::address::Address;

pub struct Config {
    beneficiary: Address,
    genesis: Genesis,
}

/// The state of the current blockchain node.
pub struct State {
    beneficiary: Address,
    genesis: Genesis,
    db: Database,
}

impl State {
    pub fn new(config: Config) -> Result<Self> {
        let db = Database::new(config.genesis.clone())?;
        Ok(State {
            beneficiary: config.beneficiary,
            genesis: config.genesis,
            db,
        })
    }
}
