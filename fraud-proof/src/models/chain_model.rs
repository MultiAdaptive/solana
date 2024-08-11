use diesel::{Insertable, Selectable};
use diesel::Queryable;
use lombok::{Getter, Setter};
use serde::{Deserialize, Serialize};

use crate::entities::chain_entity::table_chain;

#[derive(Debug, Clone, Deserialize, Serialize, Setter, Getter)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct ChainData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<i64>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = table_chain)]
pub struct ChainRow {
    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_id)]
    pub id: i64,

    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_slot)]
    pub slot: i64,

    #[diesel(sql_type = Timestamp)]
    #[diesel(column_name = column_updated_on)]
    pub updated_on: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = table_chain)]
pub struct ChainRecord {
    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_slot)]
    pub column_slot: i64,
}
