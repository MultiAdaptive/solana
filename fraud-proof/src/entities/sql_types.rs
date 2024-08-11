use postgres_types::{FromSql, ToSql};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, FromSql, ToSql)]
#[postgres(name = "TransactionMessageHeader")]
pub struct DbTransactionMessageHeader {
    pub num_required_signatures: i16,
    pub num_readonly_signed_accounts: i16,
    pub num_readonly_unsigned_accounts: i16,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromSql, ToSql)]
#[postgres(name = "CompiledInstruction")]
pub struct DbCompiledInstruction {
    pub program_id_index: i16,
    pub accounts: Vec<i16>,
    pub data: Vec<u8>,
}


#[derive(Clone, Debug, Serialize, Deserialize, FromSql, ToSql)]
#[postgres(name = "TransactionMessage")]
pub struct DbTransactionMessage {
    pub header: DbTransactionMessageHeader,
    pub account_keys: Vec<Vec<u8>>,
    pub recent_blockhash: Vec<u8>,
    pub instructions: Vec<DbCompiledInstruction>,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromSql, ToSql)]
#[postgres(name = "Reward")]
pub struct Reward {
    pub pubkey: String,
    pub lamports: i64,
    pub post_balance: i64,
    pub reward_type: RewardType,
    pub commission: Option<i16>,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromSql, ToSql)]
#[postgres(name = "reward_type")]
pub enum RewardType {
    Fee,
    Rent,
    Staking,
    Voting,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromSql, ToSql)]
#[postgres(name = "transaction_error_code")]
pub enum TransactionErrorCode {
    AccountInUse,
    AccountLoadedTwice,
    AccountNotFound,
    ProgramAccountNotFound,
    InsufficientFundsForFee,
    InvalidAccountForFee,
    AlreadyProcessed,
    BlockhashNotFound,
    InstructionError,
    CallChainTooDeep,
    MissingSignatureForFee,
    InvalidAccountIndex,
    SignatureFailure,
    InvalidProgramForExecution,
    SanitizeFailure,
    ClusterMaintenance,
    AccountBorrowOutstanding,
    WouldExceedMaxAccountCostLimit,
    WouldExceedMaxBlockCostLimit,
    UnsupportedVersion,
    InvalidWritableAccount,
    WouldExceedMaxAccountDataCostLimit,
    TooManyAccountLocks,
    AddressLookupTableNotFound,
    InvalidAddressLookupTableOwner,
    InvalidAddressLookupTableData,
    InvalidAddressLookupTableIndex,
    InvalidRentPayingAccount,
    WouldExceedMaxVoteCostLimit,
    WouldExceedAccountDataBlockLimit,
    WouldExceedAccountDataTotalLimit,
    DuplicateInstruction,
    InsufficientFundsForRent,
    MaxLoadedAccountsDataSizeExceeded,
    InvalidLoadedAccountsDataSizeLimit,
    ResanitizationNeeded,
    UnbalancedTransaction,
    ProgramExecutionTemporarilyRestricted,
}


