use crate::blockchain::database::account::AccountId;
use crate::crypto::signature::Signature;
use crate::crypto::signer::SignerSync;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    /// nonce of sender
    nonce: u64,
    /// sender address
    from: AccountId, // TODO: consider removing 'from' field to save space, as it can be derived from the signature
    /// recipient address
    to: AccountId,
    /// total amount of transaction to be sent
    value: u64,
    /// rewards for processing the transaction
    tip: u64,
    /// data payload
    data: Vec<u8>,
}

impl Transaction {
    pub fn sign<S: SignerSync, E: Fn(&Transaction) -> Vec<u8>>(
        self,
        signer: S,
        encoder: E,
    ) -> Result<Signature> {
        let encoded = encoder(&self);
        let signature = signer.eip191_sign_msg(&encoded)?;
        Ok(signature)
    }
}

pub struct TxBlock {}
