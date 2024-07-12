use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::entities::sql_types::sql_types::RewardType;

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
#[diesel(table_name = table_block)]
pub struct BlockRow {
    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_id)]
    pub id: i64,

    #[diesel(sql_type = Nullable < Int8 >)]
    #[diesel(column_name = column_slot)]
    pub slot: Option<i64>,

    #[diesel(sql_type = Nullable < Varchar >)]
    #[diesel(column_name = column_blockhash)]
    pub blockhash: Option<String>,

    #[diesel(sql_type = Nullable < Array < Nullable < RewardType >> >)]
    #[diesel(column_name = column_rewards)]
    pub rewards: Option<Vec<Option<RewardType>>>,

    #[diesel(sql_type = Nullable < Int8 >)]
    #[diesel(column_name = column_block_time)]
    pub block_time: Option<i64>,

    #[diesel(sql_type = Nullable < Int8 >)]
    #[diesel(column_name = column_block_height)]
    pub block_height: Option<i64>,

    #[diesel(sql_type = Timestamp)]
    #[diesel(column_name = column_updated_on)]
    pub updated_on: chrono::NaiveDateTime,
}

