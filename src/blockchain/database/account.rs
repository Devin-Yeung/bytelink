use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountId(String);

impl AccountId {
    pub fn from_hex(hex: &str) -> Self {
        todo!()
    }

    pub fn is_account_id() -> bool {
        todo!()
    }
}