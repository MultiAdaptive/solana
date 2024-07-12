diesel::table! {
    #[sql_name="spl_token_mint_index"]
    table_spl_token_mint_index(column_id) {
        #[sql_name = "id"]
        column_id -> Int8,

        #[sql_name = "mint_key"]
        column_mint_key -> Bytea,

        #[sql_name = "account_key"]
        column_account_key -> Bytea,

        #[sql_name = "slot"]
        column_slot -> Int8,

        #[sql_name = "updated_on"]
        column_updated_on -> Timestamp,
    }
}
