use crate::crypto::eip191::eip191_hash;
use alloy_primitives::B256;
use alloy_primitives::U256;
use anyhow::Result;

pub trait SignerSync {
    /// Signs the given hash.
    fn sign_hash_sync(&self, hash: &B256) -> alloy_signer::Result<alloy_primitives::Signature>;

    /// Signs the given message bytes using [EIP-191](https://eips.ethereum.org/EIPS/eip-191)
    /// where the message is prefixed with [`EIP191_PREFIX`][crate::crypto::constant::EIP191_PREFIX].
    fn eip191_sign_msg(&self, msg: &[u8]) -> Result<(B256, Signature)> {
        let digest = eip191_hash(msg);
        let signature = self.sign_hash_sync(&digest)?;
        Ok((digest, Signature(signature)))
    }
}

impl<T: alloy_signer::SignerSync> SignerSync for T {
    fn sign_hash_sync(&self, hash: &B256) -> alloy_signer::Result<alloy_primitives::Signature> {
        self.sign_hash_sync(hash)
    }
}

#[repr(transparent)]
pub struct Signature(alloy_primitives::Signature);

impl Signature {
    pub fn r(&self) -> U256 {
        self.0.r()
    }

    pub fn s(&self) -> U256 {
        self.0.s()
    }

    pub fn v(&self) -> bool {
        self.0.v()
    }

    /// Serialize the signature into a 65-byte array `[r (32 bytes) | s (32 bytes) | v (1 byte)]`.
    /// where `v` follows the legacy format of [EIP-155](https://eips.ethereum.org/EIPS/eip-155)
    /// which is `{0, 1} + 27` depending on the parity of the y-coordinate of the recovery point.
    pub fn as_bytes(&self) -> [u8; 65] {
        self.0.as_bytes()
    }
}
