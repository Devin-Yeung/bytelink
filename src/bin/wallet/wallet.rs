use anyhow::Result;
use bytelink::blockchain::database::account::Address;
use k256::ecdsa::SigningKey;
use k256::elliptic_curve::rand_core::OsRng;

pub struct Wallet {
    key: SigningKey,
}

impl Wallet {
    pub fn random() -> Self {
        let key = SigningKey::random(&mut OsRng);
        Wallet { key }
    }

    pub fn address(&self) -> Address {
        Address::from_private_key(&self.key)
    }

    pub fn as_hex(&self) -> String {
        format!("{:x}", self.key.to_bytes())
    }

    pub fn from_hex(hex: &str) -> Result<Self> {
        let bytes = hex::decode(hex.trim())?;
        let key = SigningKey::from_slice(&bytes)?;
        Ok(Wallet { key })
    }
}

#[cfg(test)]
mod test {
    use crate::wallet::Wallet;

    #[test]
    fn wallet_identity() {
        let wallet = Wallet::random();
        let hex = wallet.as_hex();
        let recover = Wallet::from_hex(&hex).unwrap();
        assert_eq!(wallet.address(), recover.address());
    }
}
