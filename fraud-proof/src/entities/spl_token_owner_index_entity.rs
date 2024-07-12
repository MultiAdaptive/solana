diesel::table! {
    #[sql_name="spl_token_owner_index"]
    table_spl_token_owner_index(column_id) {
        #[sql_name = "id"]
        column_id -> Int8,

        #[sql_name = "owner_key"]
        column_owner_key -> Bytea,

        #[sql_name = "account_key"]
        column_account_key -> Bytea,

        #[sql_name = "slot"]
        column_slot -> Int8,

        #[sql_name = "updated_on"]
        column_updated_on -> Timestamp,
    }
}

