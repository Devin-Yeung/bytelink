use crate::blockchain::database::account::AccountId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    chain_id: u16,
    nonce: u64,
    from_id: AccountId,
    to_id: AccountId,
    value: u64,
    tip: u64,
    data: Vec<u8>,
}
