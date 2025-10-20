use alloy_core::primitives::Address;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountId(Address);

impl Deref for AccountId {
    type Target = Address;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AccountId {
    /// Parses a checksummed ethereum address string ([EIP-55](https://eips.ethereum.org/EIPS/eip-55))
    /// that must start with "0x" into an AccountId regardless of chain id.
    pub fn parse_checksummed<S: AsRef<str>>(hex: S) -> Result<Self> {
        let inner = Address::parse_checksummed(hex, None)?;
        Ok(AccountId(inner))
    }
}

#[cfg(test)]
mod tests {
    use crate::blockchain::database::account::AccountId;

    #[test]
    fn valid_checksummed_address() {
        AccountId::parse_checksummed("0xF01813E4B85e178A83e29B8E7bF26BD830a25f32").unwrap();
    }

    #[test]
    fn invalid_checksummed_address() {
        // lowercase the first character (F -> f)
        assert!(
            AccountId::parse_checksummed("0xf01813e4b85e178a83e29b8e7bf26bd830a25f32").is_err()
        );
    }
}
