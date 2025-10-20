use crate::crypto::constant::EIP191_PREFIX;
use alloy_primitives::B256;
use alloy_primitives::U256;
use alloy_primitives::utils::keccak256;
use alloy_signer::SignerSync;
use alloy_signer_local::PrivateKeySigner;
use anyhow::Result;

#[repr(transparent)]
pub struct Signer(PrivateKeySigner);

impl Signer {
    /// Sign the given message bytes using [EIP-191](https://eips.ethereum.org/EIPS/eip-191)
    /// where the message is prefixed with [`EIP191_PREFIX`][crate::crypto::constant::EIP191_PREFIX].
    pub fn sign(&self, msg: &[u8]) -> Result<Signature> {
        let msg = Self::eip191_hash(msg);
        let sig = self.0.sign_hash_sync(&msg)?;
        Ok(Signature(sig))
    }

    /// Create an [EIP-191](https://eips.ethereum.org/EIPS/eip-191) formatted message from the given byte slice.
    /// ```text
    /// ┌────────────────────────────────┬───────────┬───────────┐
    /// │ \x19Bytelink Signed Message:\n │ len(msg)  │  payload  │
    /// └────────────────────────────────┴───────────┴───────────┘
    /// ```
    fn eip191_message<T: AsRef<[u8]>>(message: T) -> Vec<u8> {
        fn eip191_message(message: &[u8]) -> Vec<u8> {
            let len = message.len();
            let mut len_string_buffer = itoa::Buffer::new();
            let len_string = len_string_buffer.format(len);

            let mut eth_message = Vec::with_capacity(EIP191_PREFIX.len() + len_string.len() + len);
            eth_message.extend_from_slice(EIP191_PREFIX.as_bytes());
            eth_message.extend_from_slice(len_string.as_bytes());
            eth_message.extend_from_slice(message);
            eth_message
        }

        eip191_message(message.as_ref())
    }

    /// Create a Keccak-256 hash of an [EIP-191](https://eips.ethereum.org/EIPS/eip-191) formatted message.
    fn eip191_hash<T: AsRef<[u8]>>(message: T) -> B256 {
        keccak256(Self::eip191_message(message))
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
}
