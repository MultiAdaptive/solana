use diesel::Insertable;
use serde::{Deserialize, Serialize};

use crate::entities::entry_entity::table_entry;

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = table_entry)]
pub struct EntryRecord {
    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_slot)]
    pub column_slot: i64,

    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_parent_slot)]
    pub column_parent_slot: i64,

    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_entry_index)]
    pub column_entry_index: i64,

    #[diesel(sql_type = Nullable < Bytea >)]
    #[diesel(column_name = column_entry)]
    pub column_entry: Option<Vec<u8>>,

    #[diesel(sql_type = Bool)]
    #[diesel(column_name = column_is_full_slot)]
    pub column_is_full_slot: bool,

    #[diesel(sql_type = Timestamp)]
    #[diesel(column_name = column_updated_on)]
    pub column_updated_on: chrono::NaiveDateTime,
}


