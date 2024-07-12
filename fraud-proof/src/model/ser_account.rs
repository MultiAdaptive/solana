use std::str::FromStr;

use serde::{Deserialize, Serialize};
use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;

#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Debug, Clone)]
#[derive(Eq, PartialEq)]
pub struct SerAccount {
    pub lamports: u64,
    pub data: String,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: u64,
}


impl SerAccount {
    pub fn from_normal_account(account: Account) -> Self {
        Self {
            lamports: account.lamports,
            data: hex::encode(account.data),
            owner: account.owner.to_string(),
            executable: account.executable,
            rent_epoch: account.rent_epoch,
        }
    }

    pub fn to_normal_account(&self) -> Account {
        Account {
            lamports: self.lamports,
            data: hex::decode(&self.data).unwrap(),
            owner: Pubkey::from_str(&self.owner).unwrap(),
            executable: self.executable,
            rent_epoch: self.rent_epoch,
        }
    }
}

