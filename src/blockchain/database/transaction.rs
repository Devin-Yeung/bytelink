use crate::blockchain::database::account::AccountId;
use crate::crypto::signature::Signature;
use crate::crypto::signer::SignerSync;
use alloy_rlp::{RlpDecodable, RlpEncodable};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, RlpEncodable, RlpDecodable)]
pub struct Transaction {
    /// nonce of sender
    nonce: u64,
    /// recipient address
    to: AccountId,
    /// total amount of transaction to be sent
    value: u64,
    /// merge gas_limit and gas_price into tip
    tip: u64,
    /// data payload
    data: Vec<u8>,
}

impl Transaction {
    pub fn sign<S: SignerSync>(self, signer: S) -> Result<Signature> {
        todo!()
    }
}
