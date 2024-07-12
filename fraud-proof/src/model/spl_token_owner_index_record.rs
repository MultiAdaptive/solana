use diesel::Insertable;
use serde::{Deserialize, Serialize};

use crate::entities::spl_token_owner_index_entity::table_spl_token_owner_index;

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = table_spl_token_owner_index)]
pub struct SplTokenOwnerIndexRecord {
    #[diesel(sql_type = Bytea)]
    #[diesel(column_name = column_owner_key)]
    pub column_owner_key: Vec<u8>,

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

