use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
#[diesel(table_name = table_spl_token_mint_index)]
pub struct SplTokenMintIndexRow {
    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_id)]
    pub id: i64,

    #[diesel(sql_type = Bytea)]
    #[diesel(column_name = column_mint_key)]
    pub mint_key: Vec<u8>,

    #[diesel(sql_type = Bytea)]
    #[diesel(column_name = column_account_key)]
    pub account_key: Vec<u8>,

    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_slot)]
    pub slot: i64,

    #[diesel(sql_type = Timestamp)]
    #[diesel(column_name = column_updated_on)]
    pub updated_on: chrono::NaiveDateTime,
}


