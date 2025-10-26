use alloy_signer::k256::ecdsa::SigningKey;
use anyhow::Result;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
#[repr(transparent)]
pub struct Address(alloy_primitives::Address);

impl From<alloy_primitives::Address> for Address {
    fn from(addr: alloy_primitives::Address) -> Self {
        Address(addr)
    }
}

impl Address {
    pub fn random<R: Rng>(mut rng: R) -> Self {
        let bytes: [u8; 20] = rng.random();
        let inner = alloy_primitives::Address::from_slice(&bytes);
        Address(inner)
    }

    /// Parses a checksummed ethereum address string ([EIP-55](https://eips.ethereum.org/EIPS/eip-55))
    /// that must start with "0x" into an AccountId regardless of chain id.
    pub fn parse_checksummed<S: AsRef<str>>(hex: S) -> Result<Self> {
        let inner = alloy_primitives::Address::parse_checksummed(hex, None)?;
        Ok(Address(inner))
    }

    pub fn from_private_key(key: &SigningKey) -> Self {
        let inner = alloy_primitives::Address::from_private_key(key);
        Address(inner)
    }

    /// return the hex string representation of the checksummed address
    pub fn checksummed(&self) -> String {
        self.0.to_checksum(None)
    }
}

/// Hex representation tests for Address
impl std::fmt::LowerHex for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

impl std::fmt::UpperHex for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::address::Address;
    use rand::rng;

    #[test]
    fn hex_repr() {
        let address =
            Address::parse_checksummed("0xF01813E4B85e178A83e29B8E7bF26BD830a25f32").unwrap();
        assert_eq!(
            format!("{:x}", address),
            "0xf01813e4b85e178a83e29b8e7bf26bd830a25f32"
        );
        assert_eq!(
            format!("{:X}", address),
            "0xF01813E4B85E178A83E29B8E7BF26BD830A25F32"
        );
    }

    #[test]
    fn valid_checksummed_address() {
        Address::parse_checksummed("0xF01813E4B85e178A83e29B8E7bF26BD830a25f32").unwrap();
    }

    #[test]
    fn invalid_checksummed_address() {
        // lowercase the first character (F -> f)
        assert!(Address::parse_checksummed("0xf01813e4b85e178a83e29b8e7bf26bd830a25f32").is_err());
    }

    #[test]
    fn generate_random_address() {
        let addr1 = Address::random(rng());
        let addr2 = Address::random(rng());
        assert_ne!(addr1, addr2);
    }
}
