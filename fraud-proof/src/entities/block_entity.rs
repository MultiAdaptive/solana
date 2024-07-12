diesel::table! {
    use diesel::sql_types::*;
use crate::entities::sql_types::sql_types::RewardType;

    #[sql_name="block"]
    table_block (column_id) {
        #[sql_name = "id"]
        column_id -> Int8,

        #[sql_name = "slot"]
        column_slot -> Nullable<Int8>,

        #[sql_name = "blockhash"]
        column_blockhash -> Nullable<Varchar>,

        #[sql_name = "rewards"]
        column_rewards -> Nullable<Array<Nullable<RewardType>>>,

        #[sql_name = "block_time"]
        column_block_time -> Nullable<Int8>,

        #[sql_name = "block_height"]
        column_block_height -> Nullable<Int8>,

        #[sql_name = "updated_on"]
        column_updated_on -> Timestamp,
    }
}
