use std::collections::HashMap;
use std::io;

use chrono;
use diesel::{prelude::*};
use postgres_types::{FromSql, ToSql};
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

use solana_sdk::{account::Account, pubkey::Pubkey, transaction::Transaction};

use crate::account_smt::SMTAccount;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostgresConfig {
    pub host: Option<String>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub dbname: Option<String>,
    pub port: Option<u16>,
}

#[derive(Error, Debug)]
pub enum SMTerError {
    #[error("Error opening config file. Error detail: ({0}).")]
    ConfigFileOpenError(#[from] io::Error),

    #[error("Error reading config file. Error message: ({msg})")]
    ConfigFileReadError { msg: String },

    #[error("Error connecting postgresdb Error message: ({msg})")]
    DbConnectError { msg: String },

    #[error("Error starting SMT RocksDB Error message: ({msg})")]
    InitSMTDBError { msg: String },

    #[error("Error initing SMT with RocksDB Error message: ({msg})")]
    InitSMTError { msg: String },

    #[error("Error starting SMT RocksDB Error message: ({msg})")]
    InitRecordDBError { msg: String },

    #[error("Error updating SMT RocksDB Error message: ({msg})")]
    UpdatingSMTError { msg: String },

    #[error("Error query slot not match Error message: ({msg})")]
    QuerySlotNotMatchError { msg: String },

    #[error("Error query slot not exists Error message: ({msg})")]
    QuerySlotNotExistsError { msg: String },

    #[error("Error query during decoding tx by base58 Error message: ({msg})")]
    QuerytxDecodeError { msg: String },

    #[error("Error query during construct temp smt Error message: ({msg})")]
    QueryCopySMTError { msg: String },

    #[error("Error query transaction not exists Error message: ({msg})")]
    QueryTxNotExistsError { msg: String },
}

diesel::table! {
    #[sql_name="account_audit"]
    table_account_audit(column_id) {
        #[sql_name = "id"]
        column_id -> Bigint,
        #[sql_name = "pubkey"]
        column_pubkey -> Bytea,
        #[sql_name = "owner"]
        column_owner -> Bytea,
        #[sql_name = "lamports"]
        column_lamports -> Bigint,
        #[sql_name = "slot"]
        column_slot -> Bigint,
        #[sql_name = "executable"]
        column_executable -> Bool,
        #[sql_name = "rent_epoch"]
        column_rent_epoch -> Bigint,
        #[sql_name = "data"]
        column_data -> Bytea,
        #[sql_name = "write_version"]
        column_write_version -> Bigint,
        #[sql_name = "txn_signature"]
        column_txn_signature -> Nullable<Bytea>,
        #[sql_name = "updated_on"]
        column_updated_on -> Timestamp,
    }
}

#[derive(Debug, Clone, Default, Queryable, Serialize, Deserialize)]
pub struct AccountAuditRow {
    pub id: i64,
    pub pubkey: Vec<u8>,
    pub owner: Vec<u8>,
    pub lamports: i64,
    pub slot: i64,
    pub executable: bool,
    pub rent_epoch: i64,
    pub data: Vec<u8>,
    pub write_version: i64,
    pub txn_signature: Option<Vec<u8>>,
    pub updated_on: chrono::NaiveDateTime,
}

impl AccountAuditRow {
    pub fn to_smt_account(&self) -> SMTAccount {
        SMTAccount {
            pubkey: Pubkey::try_from(self.pubkey.as_slice()).unwrap(),
            lamports: self.lamports,
            owner: Pubkey::try_from(self.owner.as_slice()).unwrap(),
            executable: self.executable,
            rent_epoch: self.rent_epoch,
            data: self.data.clone(),
        }
    }
}

diesel::table! {
    #[sql_name="slot"]
    table_slot(column_slot) {
        #[sql_name = "slot"]
        column_slot -> Bigint,
        #[sql_name = "parent"]
        column_parent -> Bigint,
        #[sql_name = "status"]
        column_status -> Text,
        #[sql_name = "updated_on"]
        column_updated_on -> Timestamp,
    }
}

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct SlotRow {
    pub slot: i64,
    pub parent: i64,
    pub status: String,
    pub updated_on: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxContext {
    pub pre_accounts: HashMap<String, (String, SerAccount)>,
    pub post_accounts: HashMap<String, (String, SerAccount)>,
    pub tx: Transaction,
    pub pre_root: String,
    pub update_commitment: String,
    pub pre_account_proof: String,
    pub block_height: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SlotSummary {
    pub slot: u64,
    pub record: HashRecord,
    pub count: usize,
}

/*
* slot_i                                slot_i+1
* |                root_i-1             |               root_i
* ↓                ↓             ↓ha_i  ↓               ↓          ↓ha_i+1
* ┌----------------┬─────────────┬-----┬┬---------------┬──────────┬------┐
* │    system tx   │   user tx   │  st │|   system tx   │ user tx  │  st  │
* └----------------┴─────────────┴-----┴┴---------------┴──────────┴------┘
* ↑                ↑             ↑      ↑               ↑          ↑      ↑
* first_hash  second_hash  thrid_hash fourth_hash
*
* first_hash_i = fourth_hash_i-1
* root_i = second_hash_i+1
* ha_i = third_hash_i, third_hash is calculated within this slot, has nothing to do with SMT.
* system tx/st: are transactions that used for system, which can not be challenged.
* user tx: can be challenged
*/
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HashRecord {
    pub first_hash: String,
    pub second_hash: String,
    pub thrid_hash: String,
    pub fourth_hash: String,
}

#[derive(Clone, Debug, FromSql, ToSql)]
#[postgres(name = "TransactionMessageHeader")]
pub struct DbTransactionMessageHeader {
    pub num_required_signatures: i16,
    pub num_readonly_signed_accounts: i16,
    pub num_readonly_unsigned_accounts: i16,
}

#[derive(Clone, Debug, FromSql, ToSql)]
#[postgres(name = "CompiledInstruction")]
pub struct DbCompiledInstruction {
    pub program_id_index: i16,
    pub accounts: Vec<i16>,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, FromSql, ToSql)]
#[postgres(name = "TransactionMessage")]
pub struct DbTransactionMessage {
    pub header: DbTransactionMessageHeader,
    pub account_keys: Vec<Vec<u8>>,
    pub recent_blockhash: Vec<u8>,
    pub instructions: Vec<DbCompiledInstruction>,
}
