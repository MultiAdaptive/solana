use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
#[diesel(table_name = table_entry)]
pub struct EntryRow {
    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_id)]
    pub id: i64,

    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_slot)]
    pub slot: i64,

    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_parent_slot)]
    pub parent_slot: i64,

    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_entry_index)]
    pub entry_index: i64,

    #[diesel(sql_type = Nullable < Bytea >)]
    #[diesel(column_name = column_entry)]
    pub entry: Option<Vec<u8>>,

    #[diesel(sql_type = Bool)]
    #[diesel(column_name = column_is_full_slot)]
    pub is_full_slot: bool,

    #[diesel(sql_type = Timestamp)]
    #[diesel(column_name = column_updated_on)]
    pub updated_on: chrono::NaiveDateTime,
}

