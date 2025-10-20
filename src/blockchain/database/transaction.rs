use crate::blockchain::database::account::AccountId;
use crate::crypto::signature::{Signature, SignerSync};
use alloy_primitives::B256;
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
    /// Encode the transaction with RLP encoding and packed into an [EIP-191](https://eips.ethereum.org/EIPS/eip-191)
    /// formatted message. Finally, sign the message with the given signer.
    pub fn sign<S: SignerSync>(self, signer: S) -> Result<SignedTx> {
        // encode the transaction using RLP
        let encoded = alloy_rlp::encode(&self);
        // create keccak256 hash of the EIP-191 formatted message
        let (digest, signature) = signer.eip191_sign_msg(&encoded)?;
        Ok(SignedTx {
            tx: self,
            digest,
            signature,
        })
    }
}

pub struct SignedTx {
    pub(crate) tx: Transaction,
    pub(crate) digest: B256,
    pub(crate) signature: Signature,
}

impl SignedTx {
    pub fn verify(&self) -> Result<Transaction> {
        todo!()
    }
}
