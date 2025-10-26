use crate::address::Address;
use crate::eip191::eip191_hash;
use crate::signature::Signature;
use alloy_primitives::B256;
use anyhow::Result;

pub trait SignerSync {
    /// Signs the given hash.
    fn sign_hash_sync(&self, hash: &B256) -> Result<Signature>;

    /// Signs the given message bytes using [EIP-191](https://eips.ethereum.org/EIPS/eip-191)
    /// where the message is prefixed with the bytelink specific [`EIP191_PREFIX`][crate::crypto::constant::EIP191_PREFIX].
    fn eip191_sign_msg(&self, msg: &[u8]) -> Result<Signature> {
        let digest = eip191_hash(msg);
        self.sign_hash_sync(&digest)
    }
}

impl<T: alloy_signer::SignerSync> SignerSync for T {
    fn sign_hash_sync(&self, hash: &B256) -> Result<Signature> {
        let sig = self.sign_hash_sync(hash)?;
        Ok(Signature(sig))
    }
}

#[repr(transparent)]
pub struct Signer(alloy_signer_local::PrivateKeySigner);

impl SignerSync for Signer {
    fn sign_hash_sync(&self, hash: &B256) -> Result<Signature> {
        self.0.sign_hash_sync(hash)
    }
}

impl Signer {
    pub fn random() -> Self {
        Self(alloy_signer_local::PrivateKeySigner::random())
    }

    pub fn address(&self) -> Address {
        let address = self.0.address();
        Address::from(address)
    }
}
