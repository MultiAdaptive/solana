diesel::table! {
    #[sql_name="entry"]
    table_entry(column_id) {
        #[sql_name = "id"]
        column_id -> Int8,

        #[sql_name = "slot"]
        column_slot -> Int8,

        #[sql_name = "parent_slot"]
        column_parent_slot -> Int8,

        #[sql_name = "entry_index"]
        column_entry_index -> Int8,

        #[sql_name = "entry"]
        column_entry -> Nullable<Bytea>,

        #[sql_name = "is_full_slot"]
        column_is_full_slot -> Bool,

        #[sql_name = "updated_on"]
        column_updated_on -> Timestamp,
    }
}
