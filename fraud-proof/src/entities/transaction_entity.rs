diesel::table! {
    use diesel::sql_types::*;
    use crate::entities::sql_types::sql_types::TransactionMessageType;
    use crate::entities::sql_types::sql_types::LoadedMessageV0Type;
    use crate::entities::sql_types::sql_types::TransactionStatusMetaType;

    #[sql_name="transaction"]
    table_transaction (column_id) {
        #[sql_name = "id"]
        column_id -> Int8,

        #[sql_name = "slot"]
        column_slot -> Int8,

        #[sql_name = "signature"]
        column_signature -> Bytea,

        #[sql_name = "is_vote"]
        column_is_vote -> Bool,

        #[sql_name = "message_type"]
        column_message_type -> Nullable<Int2>,

        #[sql_name = "legacy_message"]
        column_legacy_message -> Nullable<TransactionMessageType>,

        #[sql_name = "v0_loaded_message"]
        column_v0_loaded_message -> Nullable<LoadedMessageV0Type>,

        #[sql_name = "signatures"]
        column_signatures -> Nullable<Array<Nullable<Bytea>>>,

        #[sql_name = "message_hash"]
        column_message_hash -> Nullable<Bytea>,

        #[sql_name = "meta"]
        column_meta -> Nullable<TransactionStatusMetaType>,

        #[sql_name = "write_version"]
        column_write_version -> Nullable<Int8>,

        #[sql_name = "index"]
        column_index -> Int8,

        #[sql_name = "updated_on"]
        column_updated_on -> Timestamp,
    }
}

