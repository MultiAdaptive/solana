use diesel::Insertable;
use serde::{Deserialize, Serialize};

use crate::entities::account_entity::table_account;

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = table_account)]
pub struct AccountRecord {
    #[diesel(sql_type = Bytea)]
    #[diesel(column_name = column_pubkey)]
    pub column_pubkey: Vec<u8>,

    #[diesel(sql_type = Nullable < Bytea >)]
    #[diesel(column_name = column_owner)]
    pub column_owner: Option<Vec<u8>>,

    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_lamports)]
    pub column_lamports: i64,

    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_slot)]
    pub column_slot: i64,

    #[diesel(sql_type = Bool)]
    #[diesel(column_name = column_executable)]
    pub column_executable: bool,

    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_rent_epoch)]
    pub column_rent_epoch: i64,

    #[diesel(sql_type = Nullable < Bytea >)]
    #[diesel(column_name = column_data)]
    pub column_data: Option<Vec<u8>>,

    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_write_version)]
    pub column_write_version: i64,

    #[diesel(sql_type = Nullable < Bytea >)]
    #[diesel(column_name = column_txn_signature)]
    pub column_txn_signature: Option<Vec<u8>>,

    #[diesel(sql_type = Timestamp)]
    #[diesel(column_name = column_updated_on)]
    pub column_updated_on: chrono::NaiveDateTime,
}

