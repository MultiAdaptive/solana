use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerBrief {
    pub slot: u64,
    pub root_hash: String,
    pub hash_account: String,
    pub transaction_number: u32,
}

