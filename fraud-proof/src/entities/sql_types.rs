use postgres_types::{FromSql, ToSql};

pub mod sql_types {
    use serde_derive::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "LoadedMessageV0"))]
    pub struct LoadedMessageV0Type;


    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "Reward"))]
    pub struct RewardType;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "TransactionMessage"))]
    pub struct TransactionMessageType;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "TransactionStatusMeta"))]
    pub struct TransactionStatusMetaType;
}

#[derive(Clone, Debug, FromSql, ToSql)]
#[postgres(name = "TransactionMessageHeader")]
pub struct DbTransactionMessageHeader {
    pub num_required_signatures: i16,
    pub num_readonly_signed_accounts: i16,
    pub num_readonly_unsigned_accounts: i16,
}

#[derive(Clone, Debug, FromSql, ToSql)]
#[postgres(name = "CompiledInstruction")]
pub struct DbCompiledInstruction {
    pub program_id_index: i16,
    pub accounts: Vec<i16>,
    pub data: Vec<u8>,
}


#[derive(Clone, Debug, FromSql, ToSql)]
#[postgres(name = "TransactionMessage")]
pub struct DbTransactionMessage {
    pub header: DbTransactionMessageHeader,
    pub account_keys: Vec<Vec<u8>>,
    pub recent_blockhash: Vec<u8>,
    pub instructions: Vec<DbCompiledInstruction>,
}
