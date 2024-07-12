use diesel::Insertable;
use serde::{Deserialize, Serialize};

use crate::entities::spl_token_mint_index_entity::table_spl_token_mint_index;

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = table_spl_token_mint_index)]
pub struct SplTokenMintIndexRecord {
    #[diesel(sql_type = Bytea)]
    #[diesel(column_name = column_mint_key)]
    pub column_mint_key: Vec<u8>,

    #[diesel(sql_type = Bytea)]
    #[diesel(column_name = column_account_key)]
    pub column_account_key: Vec<u8>,

    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_slot)]
    pub column_slot: i64,

    #[diesel(sql_type = Timestamp)]
    #[diesel(column_name = column_updated_on)]
    pub column_updated_on: chrono::NaiveDateTime,
}

