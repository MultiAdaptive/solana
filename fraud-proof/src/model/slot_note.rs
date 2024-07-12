use diesel::Insertable;
use serde::{Deserialize, Serialize};

use crate::entities::slot_entity::table_slot;

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = table_slot)]
pub struct SlotNote {
    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_slot)]
    pub column_slot: i64,

    #[diesel(sql_type = Varchar)]
    #[diesel(column_name = column_status)]
    pub column_status: String,

    #[diesel(sql_type = Timestamp)]
    #[diesel(column_name = column_updated_on)]
    pub column_updated_on: chrono::NaiveDateTime,
}
