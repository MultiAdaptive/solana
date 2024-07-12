use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
// #[diesel(table_name = table_transaction)]
pub struct TransactionRow {
    // #[diesel(sql_type = Int8)]
    // #[diesel(column_name = column_id)]
    // pub id: i64,
    //
    // #[diesel(sql_type = Nullable < Int8 >)]
    // #[diesel(column_name = column_slot)]
    // pub slot: Option<i64>,

    // #[diesel(sql_type = Bytea)]
    // #[diesel(column_name = column_signature)]
    // pub signature: Vec<u8>,

    // #[diesel(sql_type = Bool)]
    // #[diesel(column_name = column_is_vote)]
    // pub is_vote: bool,

    // #[diesel(sql_type = Nullable < Int2 >)]
    // #[diesel(column_name = column_message_type)]
    // pub message_type: Option<i16>,
    //
    // #[diesel(sql_type = Nullable < TransactionMessageType >)]
    // #[diesel(column_name = column_legacy_message)]
    // pub legacy_message: Option<TransactionMessageType>,
    //
    // #[diesel(sql_type = Nullable < LoadedMessageV0Type >)]
    // #[diesel(column_name = column_v0_loaded_message)]
    // pub v0_loaded_message: Option<LoadedMessageV0Type>,

    // #[diesel(sql_type = Nullable < Array < Nullable < Bytea >> >)]
    // #[diesel(column_name = column_signatures)]
    // pub signatures: Option<Vec<Option<Vec<u8>>>>,

    // #[diesel(sql_type = Nullable < Bytea >)]
    // #[diesel(column_name = column_message_hash)]
    // pub message_hash: Option<Vec<u8>>,
    //
    // #[diesel(sql_type = Nullable < TransactionStatusMetaType >)]
    // #[diesel(column_name = column_meta)]
    // pub meta: Option<TransactionStatusMetaType>,
    //
    // #[diesel(sql_type = Nullable < Int8 >)]
    // #[diesel(column_name = column_write_version)]
    // pub write_version: Option<i64>,
    //
    #[diesel(sql_type = Int8)]
    // #[diesel(column_name = column_index)]
    pub index: i64,
    //
    // #[diesel(sql_type = Timestamp)]
    // #[diesel(column_name = column_updated_on)]
    // pub updated_on: chrono::NaiveDateTime,
}


