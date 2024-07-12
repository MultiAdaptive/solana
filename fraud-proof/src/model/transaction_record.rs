use diesel::Insertable;
use serde::{Deserialize, Serialize};

use crate::entities::transaction_entity::table_transaction;

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = table_transaction)]
pub struct TransactionRecord {
    #[diesel(sql_type = Nullable < Int8 >)]
    #[diesel(column_name = column_slot)]
    pub column_slot: Option<i64>,

    #[diesel(sql_type = Bytea)]
    #[diesel(column_name = column_signature)]
    pub column_signature: Vec<u8>,

    #[diesel(sql_type = Bool)]
    #[diesel(column_name = column_is_vote)]
    pub column_is_vote: bool,

    #[diesel(sql_type = Nullable < Int2 >)]
    #[diesel(column_name = column_message_type)]
    pub column_message_type: Option<i16>,

    // #[diesel(sql_type = Nullable < TransactionMessageType >)]
    // #[diesel(column_name = column_legacy_message)]
    // pub column_legacy_message: Option<TransactionMessageType>,

    // #[diesel(sql_type = Nullable < LoadedMessageV0Type >)]
    // #[diesel(column_name = column_v0_loaded_message)]
    // pub column_v0_loaded_message: Option<LoadedMessageV0Type>,

    #[diesel(sql_type = Nullable < Array < Nullable < Bytea >> >)]
    #[diesel(column_name = column_signatures)]
    pub column_signatures: Option<Vec<Option<Vec<u8>>>>,

    #[diesel(sql_type = Nullable < Bytea >)]
    #[diesel(column_name = column_message_hash)]
    pub column_message_hash: Option<Vec<u8>>,

    // #[diesel(sql_type = Nullable < TransactionStatusMetaType >)]
    // #[diesel(column_name = column_meta)]
    // pub column_meta: Option<TransactionStatusMetaType>,

    #[diesel(sql_type = Nullable < Int8 >)]
    #[diesel(column_name = column_write_version)]
    pub column_write_version: Option<i64>,

    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_index)]
    pub column_index: i64,

    #[diesel(sql_type = Timestamp)]
    #[diesel(column_name = column_updated_on)]
    pub column_updated_on: chrono::NaiveDateTime,
}

