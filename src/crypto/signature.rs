use crate::blockchain::database::account::AccountId;
use alloy_primitives::U256;
use anyhow::Result;

#[repr(transparent)]
pub struct Signature(pub(crate) alloy_primitives::Signature);

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

    /// Recover the address from the payload message that use [EIP-191](https://eips.ethereum.org/EIPS/eip-191) formatting.
    /// Where the prefix is the bytelink specific [`EIP191_PREFIX`][crate::crypto::constant::EIP191_PREFIX].
    pub fn recover_address_from_msg(&self, msg: &[u8]) -> Result<AccountId> {
        let digest = crate::crypto::eip191::eip191_hash(msg);
        let address = self.0.recover_address_from_prehash(&digest)?;
        Ok(address.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::signer::{Signer, SignerSync};

    #[test]
    fn sign_and_recover() {
        let signer = Signer::random();
        let message = b"Hello, Bytelink!";

        let signature = signer.eip191_sign_msg(message).unwrap();

        let recovered_address = signature.recover_address_from_msg(message).unwrap();
        let signer_address = signer.address();

        assert_eq!(recovered_address, signer_address);
    }
}
