use bytelink_crypto::address::Address;

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
