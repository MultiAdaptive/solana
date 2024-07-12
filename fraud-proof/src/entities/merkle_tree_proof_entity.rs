diesel::table! {
    #[sql_name="merkle_tree_proof"]
    table_merkle_tree_proof(column_id) {
        #[sql_name = "id"]
        column_id -> Int8,

        #[sql_name = "slot"]
        column_slot -> Int8,

        #[sql_name = "root_hash"]
        column_root_hash -> Nullable<Varchar>,

        #[sql_name = "updated_on"]
        column_updated_on -> Timestamp,
    }
}

