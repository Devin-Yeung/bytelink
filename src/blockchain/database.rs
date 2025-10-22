use crate::blockchain::database::account::{Account, Address};
use crate::blockchain::genesis::Genesis;
use anyhow::Result;
use std::collections::HashMap;

pub mod account;
pub mod transaction;

pub struct Database {
    genesis: Genesis,
    accounts: HashMap<Address, Account>,
}

impl Database {
    pub fn new(genesis: Genesis) -> Result<Database> {
        let mut accounts = HashMap::new();

        for (addr, balance) in &genesis.balances {
            let address = Address::parse_checksummed(addr)?;
            let account = Account::new(address.clone(), *balance);
            accounts.insert(address, account);
        }

        Ok(Database { genesis, accounts })
    }

    pub fn query<T: AsRef<Address>>(&self, address: T) -> Option<&Account> {
        self.accounts.get(address.as_ref())
    }
}
