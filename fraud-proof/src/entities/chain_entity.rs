diesel::table! {
    use diesel::sql_types::*;

    #[sql_name="chain"]
    table_chain(column_id) {
        #[sql_name = "id"]
        column_id -> Int8,

        #[sql_name = "slot"]
        column_slot -> Int8,

        #[sql_name = "updated_on"]
        column_updated_on -> Timestamp,
    }
}

