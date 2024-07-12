diesel::table! {
    #[sql_name="replay"]
    table_replay(column_id) {
        #[sql_name = "id"]
        column_id -> Int8,

        #[sql_name = "slot"]
        column_slot -> Int8,

        #[sql_name = "entry_index"]
        column_entry_index -> Int8,

        #[sql_name = "updated_on"]
        column_updated_on -> Timestamp,
    }
}

