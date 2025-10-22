use crate::blockchain::database::account::{Account, Address};
use crate::blockchain::genesis::Genesis;
use anyhow::Result;
use dashmap::DashMap;
use dashmap::mapref::one::Ref;

pub mod account;
pub mod transaction;

/// The in-memory, concurrent safe database for blockchain accounts.
pub struct Database {
    genesis: Genesis,
    accounts: DashMap<Address, Account>,
}

impl Database {
    pub fn new(genesis: Genesis) -> Result<Database> {
        let accounts = DashMap::new();

        for (addr, balance) in &genesis.balances {
            let address = Address::parse_checksummed(addr)?;
            let account = Account::new(address.clone(), *balance);
            accounts.insert(address, account);
        }

        Ok(Database { genesis, accounts })
    }

    pub fn query<T: AsRef<Address>>(&self, address: T) -> Option<Ref<'_, Address, Account>> {
        self.accounts.get(address.as_ref())
    }
}
