use anyhow::Result;
use bytelink_crypto::address::Address;
use bytelink_crypto::signature::Signature;
use bytelink_crypto::signer::SignerSync;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    /// nonce of sender
    pub nonce: u64,
    /// sender address
    pub from: Address, // TODO: consider removing 'from' field to save space, as it can be derived from the signature
    /// recipient address
    pub to: Address,
    /// total amount of transaction to be sent
    pub value: u64,
    /// rewards for processing the transaction
    pub tip: u64,
    /// data payload
    pub data: Vec<u8>,
}

impl Transaction {
    pub fn sign_by<S: SignerSync>(self, signer: &S) -> Result<SignedTx> {
        let encoded = serde_json::to_vec(&self)?;
        let signature = signer.eip191_sign_msg(&encoded)?;
        Ok(SignedTx {
            transaction: self,
            signature,
        })
    }
}

pub struct SignedTx {
    pub transaction: Transaction,
    signature: Signature,
}

impl SignedTx {
    pub fn verify(&self) -> Result<()> {
        let encoded = serde_json::to_vec(&self.transaction)?;
        let recovered_address = self.signature.recover_address_from_msg(&encoded)?;

        if recovered_address != self.transaction.from {
            anyhow::bail!(
                "Invalid transaction signature: recovered address does not match sender address"
            );
        }

        if recovered_address == self.transaction.to {
            anyhow::bail!("Invalid transaction: sender and recipient addresses cannot be the same");
        }

        Ok(())
    }
}

/// Define how a transection looks like in a block
pub struct BlockTx {
    tx: SignedTx,
    pub timestamp: u64,
    pub gas_price: u64,
    pub gas_units: u64,
}

impl BlockTx {
    pub fn new(tx: SignedTx, gas_price: u64, gas_units: u64) -> Self {
        BlockTx {
            tx,
            timestamp: Timestamp::now().as_millisecond() as u64,
            gas_price,
            gas_units,
        }
    }

    pub fn tx(&self) -> &Transaction {
        &self.tx.transaction
    }
}

#[cfg(test)]
mod tests {
    use crate::database::transaction::Transaction;
    use bytelink_crypto::signer::Signer;

    #[test]
    pub fn naive_valid_tx() {
        let alice = Signer::random();
        let bob = Signer::random();

        let tx = Transaction {
            nonce: 0,
            from: alice.address(),
            to: bob.address(),
            value: 100,
            tip: 1,
            data: vec![],
        };

        let signed_tx = tx.sign_by(&alice).unwrap();

        assert!(signed_tx.verify().is_ok());
    }

    #[test]
    pub fn send_to_self_invalid_tx() {
        let alice = Signer::random();

        let tx = Transaction {
            nonce: 0,
            from: alice.address(),
            to: alice.address(),
            value: 100,
            tip: 1,
            data: vec![],
        };

        let signed_tx = tx.sign_by(&alice).unwrap();

        assert!(signed_tx.verify().is_err());
    }

    #[test]
    pub fn bad_signature_invalid_tx() {
        let alice = Signer::random();
        let bob = Signer::random();
        let charlie = Signer::random();

        let tx = Transaction {
            nonce: 0,
            from: alice.address(),
            to: bob.address(),
            value: 100,
            tip: 1,
            data: vec![],
        };

        let signed_tx = tx.sign_by(&charlie).unwrap();

        assert!(signed_tx.verify().is_err());
    }
}
