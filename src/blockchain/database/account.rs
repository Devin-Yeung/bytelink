use alloy_core::primitives::Address;
use alloy_primitives::bytes::BufMut;
use alloy_rlp::{Decodable, Encodable};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[repr(transparent)]
pub struct AccountId(Address);

impl Deref for AccountId {
    type Target = Address;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Encodable for AccountId {
    fn encode(&self, out: &mut dyn BufMut) {
        self.0.encode(out)
    }
}

impl Decodable for AccountId {
    fn decode(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        let address = Address::decode(buf)?;
        Ok(AccountId(address))
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
    use alloy_rlp::Decodable;

    #[test]
    fn rlp_identity() {
        let account =
            AccountId::parse_checksummed("0xF01813E4B85e178A83e29B8E7bF26BD830a25f32").unwrap();

        let encoded = alloy_rlp::encode(&account);
        let decoded: AccountId = AccountId::decode(&mut encoded.as_slice()).unwrap();

        assert_eq!(account, decoded);
    }

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
