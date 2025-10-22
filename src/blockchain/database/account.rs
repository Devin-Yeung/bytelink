use anyhow::Result;
use k256::ecdsa::SigningKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
#[repr(transparent)]
pub struct Address(alloy_primitives::Address);

impl From<alloy_primitives::Address> for Address {
    fn from(addr: alloy_primitives::Address) -> Self {
        Address(addr)
    }
}

impl Address {
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

pub struct Account {
    address: Address,
    nonce: u64,
    balance: u64,
}

impl Account {
    pub fn new(address: Address, balance: u64) -> Self {
        Account {
            address,
            nonce: 0,
            balance,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::blockchain::database::account::Address;

    #[test]
    fn valid_checksummed_address() {
        Address::parse_checksummed("0xF01813E4B85e178A83e29B8E7bF26BD830a25f32").unwrap();
    }

    #[test]
    fn invalid_checksummed_address() {
        // lowercase the first character (F -> f)
        assert!(Address::parse_checksummed("0xf01813e4b85e178a83e29b8e7bf26bd830a25f32").is_err());
    }
}
