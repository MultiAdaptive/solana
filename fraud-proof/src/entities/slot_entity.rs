diesel::table! {
    #[sql_name="slot"]
    table_slot(column_id) {
        #[sql_name = "id"]
        column_id -> Int8,

        #[sql_name = "slot"]
        column_slot -> Int8,

        #[sql_name = "parent"]
        column_parent -> Nullable<Int8>,

        #[sql_name = "status"]
        column_status -> Varchar,

        #[sql_name = "updated_on"]
        column_updated_on -> Timestamp,
    }
}

