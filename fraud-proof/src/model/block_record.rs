use diesel::Insertable;
use serde::{Deserialize, Serialize};

use crate::entities::block_entity::table_block;

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = table_block)]
pub struct BlockRecord {
    #[diesel(sql_type = Nullable < Int8 >)]
    #[diesel(column_name = column_slot)]
    pub column_slot: Option<i64>,

    #[diesel(sql_type = Nullable < Varchar >)]
    #[diesel(column_name = column_blockhash)]
    pub column_blockhash: Option<String>,

    // #[diesel(sql_type = Nullable < Array < Nullable < RewardType >> >)]
    // #[diesel(column_name = column_rewards)]
    // pub column_rewards: Option<Vec<Option<RewardType>>>,

    #[diesel(sql_type = Nullable < Int8 >)]
    #[diesel(column_name = column_block_time)]
    pub column_block_time: Option<i64>,

    #[diesel(sql_type = Nullable < Int8 >)]
    #[diesel(column_name = column_block_height)]
    pub column_block_height: Option<i64>,

    #[diesel(sql_type = Timestamp)]
    #[diesel(column_name = column_updated_on)]
    pub column_updated_on: chrono::NaiveDateTime,
}
