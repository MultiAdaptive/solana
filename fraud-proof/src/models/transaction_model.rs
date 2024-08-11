use crate::entities::sql_types::DbTransactionMessage;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRow {
    pub slot: i64,

    pub legacy_message: DbTransactionMessage,

    pub signatures: Vec<Vec<u8>>,
}

