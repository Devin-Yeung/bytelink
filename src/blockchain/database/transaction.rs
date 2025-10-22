use crate::blockchain::database::account::Address;
use crate::crypto::signature::Signature;
use crate::crypto::signer::SignerSync;
use anyhow::Result;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    /// nonce of sender
    nonce: u64,
    /// sender address
    from: Address, // TODO: consider removing 'from' field to save space, as it can be derived from the signature
    /// recipient address
    to: Address,
    /// total amount of transaction to be sent
    value: u64,
    /// rewards for processing the transaction
    tip: u64,
    /// data payload
    data: Vec<u8>,
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
    pub signature: Signature,
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

pub struct BlockTx {
    tx: SignedTx,
    timestamp: u64,
    gas_price: u64,
    gas_units: u64,
}

#[cfg(test)]
mod tests {
    use crate::crypto::signer::{Signer, SignerSync};

    #[test]
    pub fn naive_valid_tx() {
        let alice = Signer::random();
        let bob = Signer::random();

        let tx = crate::blockchain::database::transaction::Transaction {
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

        let tx = crate::blockchain::database::transaction::Transaction {
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

        let tx = crate::blockchain::database::transaction::Transaction {
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
