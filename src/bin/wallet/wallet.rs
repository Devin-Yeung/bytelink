use crate::cli::CreateOpts;
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
}

pub fn generate_wallet(opts: &CreateOpts) {
    let wallet = Wallet::random();
    println!("Wallet generated: {}", wallet.address().checksummed());
    // TODO: save to file if needed
}
