/**
 * This plugin implementation for PostgreSQL requires the following tables
 */
-- The table storing accounts


CREATE TABLE account
(
    id            bigserial PRIMARY KEY,
    pubkey        BYTEA UNIQUE NOT NULL,
    owner         BYTEA,
    lamports      BIGINT    NOT NULL,
    slot          BIGINT    NOT NULL,
    executable    BOOL      NOT NULL,
    rent_epoch    BIGINT    NOT NULL,
    data          BYTEA,
    write_version BIGINT    NOT NULL,
    txn_signature BYTEA,
    updated_on    TIMESTAMP NOT NULL
);

CREATE INDEX index_account_owner ON account (owner);

CREATE INDEX index_account_slot ON account (slot);

-- The table storing slot information
CREATE TABLE slot
(
    id         bigserial PRIMARY KEY,
    slot       BIGINT UNIQUE NOT NULL,
    parent     BIGINT,
    status     VARCHAR(16) NOT NULL,
    updated_on TIMESTAMP   NOT NULL
);

CREATE INDEX index_slot_parent ON slot (parent);

CREATE TABLE merkle_tree
(
    id         bigserial PRIMARY KEY,
    slot       BIGINT    UNIQUE NOT NULL,
    root_hash  VARCHAR(256) DEFAULT '',
    hash_account       VARCHAR(256)  DEFAULT '',
    transaction_number INT           DEFAULT 0,
    updated_on TIMESTAMP NOT NULL
);
CREATE INDEX index_merkle_tree_root_hash ON merkle_tree (root_hash);
CREATE INDEX index_merkle_tree_hash_account ON merkle_tree (hash_account);

-- Types for Transactions

Create TYPE "TransactionErrorCode" AS ENUM (
    'AccountInUse',
    'AccountLoadedTwice',
    'AccountNotFound',
    'ProgramAccountNotFound',
    'InsufficientFundsForFee',
    'InvalidAccountForFee',
    'AlreadyProcessed',
    'BlockhashNotFound',
    'InstructionError',
    'CallChainTooDeep',
    'MissingSignatureForFee',
    'InvalidAccountIndex',
    'SignatureFailure',
    'InvalidProgramForExecution',
    'SanitizeFailure',
    'ClusterMaintenance',
    'AccountBorrowOutstanding',
    'WouldExceedMaxAccountCostLimit',
    'WouldExceedMaxBlockCostLimit',
    'UnsupportedVersion',
    'InvalidWritableAccount',
    'WouldExceedMaxAccountDataCostLimit',
    'TooManyAccountLocks',
    'AddressLookupTableNotFound',
    'InvalidAddressLookupTableOwner',
    'InvalidAddressLookupTableData',
    'InvalidAddressLookupTableIndex',
    'InvalidRentPayingAccount',
    'WouldExceedMaxVoteCostLimit',
    'WouldExceedAccountDataBlockLimit',
    'WouldExceedAccountDataTotalLimit',
    'DuplicateInstruction',
    'InsufficientFundsForRent',
    'MaxLoadedAccountsDataSizeExceeded',
    'InvalidLoadedAccountsDataSizeLimit',
    'ResanitizationNeeded',
    'UnbalancedTransaction',
    'ProgramExecutionTemporarilyRestricted'
    );

CREATE TYPE "TransactionError" AS
(
    error_code   "TransactionErrorCode",
    error_detail VARCHAR(256)
);

CREATE TYPE "CompiledInstruction" AS
(
    program_id_index SMALLINT,
    accounts         SMALLINT[],
    data             BYTEA
);

CREATE TYPE "InnerInstructions" AS
(
    index        SMALLINT,
    instructions "CompiledInstruction"[]
);

CREATE TYPE "TransactionTokenBalance" AS
(
    account_index   SMALLINT,
    mint            VARCHAR(44),
    ui_token_amount DOUBLE PRECISION,
    owner           VARCHAR(44)
);

Create TYPE "RewardType" AS ENUM (
    'Fee',
    'Rent',
    'Staking',
    'Voting'
    );

CREATE TYPE "Reward" AS
(
    pubkey       VARCHAR(44),
    lamports     BIGINT,
    post_balance BIGINT,
    reward_type  "RewardType",
    commission   SMALLINT
);

CREATE TYPE "TransactionStatusMeta" AS
(
    error               "TransactionError",
    fee                 BIGINT,
    pre_balances        BIGINT[],
    post_balances       BIGINT[],
    inner_instructions  "InnerInstructions"[],
    log_messages        TEXT[],
    pre_token_balances  "TransactionTokenBalance"[],
    post_token_balances "TransactionTokenBalance"[],
    rewards             "Reward"[]
);

CREATE TYPE "TransactionMessageHeader" AS
(
    num_required_signatures        SMALLINT,
    num_readonly_signed_accounts   SMALLINT,
    num_readonly_unsigned_accounts SMALLINT
);

CREATE TYPE "TransactionMessage" AS
(
    header           "TransactionMessageHeader",
    account_keys     BYTEA[],
    recent_blockhash BYTEA,
    instructions     "CompiledInstruction"[]
);

CREATE TYPE "TransactionMessageAddressTableLookup" AS
(
    account_key      BYTEA,
    writable_indexes SMALLINT[],
    readonly_indexes SMALLINT[]
);

CREATE TYPE "TransactionMessageV0" AS
(
    header                "TransactionMessageHeader",
    account_keys          BYTEA[],
    recent_blockhash      BYTEA,
    instructions          "CompiledInstruction"[],
    address_table_lookups "TransactionMessageAddressTableLookup"[]
);

CREATE TYPE "LoadedAddresses" AS
(
    writable BYTEA[],
    readonly BYTEA[]
);

CREATE TYPE "LoadedMessageV0" AS
(
    message          "TransactionMessageV0",
    loaded_addresses "LoadedAddresses"
);

-- The table storing transactions
CREATE TABLE transaction
(
    id                bigserial PRIMARY KEY,
    slot              BIGINT    NOT NULL,
    signature         BYTEA     NOT NULL,
    is_vote           BOOL      NOT NULL,
    message_type      SMALLINT, -- 0: legacy, 1: v0 message
    legacy_message    "TransactionMessage",
    v0_loaded_message "LoadedMessageV0",
    signatures        BYTEA[],
    message_hash      BYTEA,
    meta              "TransactionStatusMeta",
    index             BIGINT    NOT NULL,
    write_version     BIGINT,
    updated_on        TIMESTAMP NOT NULL,
    CONSTRAINT unique_transaction_slot_signature UNIQUE (slot, signature)
);

CREATE INDEX index_transaction_slot ON transaction (slot);
CREATE INDEX index_transaction_index ON transaction (index);

-- The table storing block metadata
CREATE TABLE block
(
    id           bigserial PRIMARY KEY,
    slot         BIGINT UNIQUE NOT NULL,
    blockhash    VARCHAR(44),
    rewards      "Reward"[],
    block_time   BIGINT,
    block_height BIGINT,
    updated_on   TIMESTAMP NOT NULL
);

CREATE INDEX index_block_blockhash ON block (blockhash);
CREATE INDEX index_block_block_height ON block (block_height);

-- The table storing spl token owner to account indexes
CREATE TABLE spl_token_owner_index
(
    id           bigserial PRIMARY KEY,
    owner_key   BYTEA  NOT NULL,
    account_key BYTEA  NOT NULL,
    slot        BIGINT NOT NULL
);

CREATE INDEX index_spl_token_owner_index_owner_key ON spl_token_owner_index (owner_key);
CREATE UNIQUE INDEX unique_spl_token_owner_index_owner_pair ON spl_token_owner_index (owner_key, account_key);
CREATE INDEX index_spl_token_owner_index_slot ON spl_token_owner_index (slot);

-- The table storing spl mint to account indexes
CREATE TABLE spl_token_mint_index
(
    id          bigserial PRIMARY KEY,
    mint_key    BYTEA  NOT NULL,
    account_key BYTEA  NOT NULL,
    slot        BIGINT NOT NULL
);

CREATE INDEX index_spl_token_mint_index_mint_key ON spl_token_mint_index (mint_key);
CREATE UNIQUE INDEX unique_spl_token_mint_index_mint_pair ON spl_token_mint_index (mint_key, account_key);
CREATE INDEX index_spl_token_mint_index_slot ON spl_token_mint_index (slot);

CREATE TABLE IF NOT EXISTS entry
(
    id           bigserial PRIMARY KEY,
    slot         BIGINT    NOT NULL,
    entry_index  BIGINT    NOT NULL,
    num_hashes   BIGINT    NOT NULL,
    entry        BYTEA,
    executed_transaction_count   BIGINT    NOT NULL,
    starting_transaction_index   BIGINT    NOT NULL,
    updated_on   TIMESTAMP NOT NULL
);

CREATE INDEX index_entry_slot_entry_index ON entry (slot, entry_index);
CREATE INDEX index_entry_slot ON entry (slot);

CREATE TABLE IF NOT EXISTS untrusted_entry
(
    id           bigserial PRIMARY KEY,
    slot         BIGINT    NOT NULL,
    parent_slot  BIGINT    NOT NULL,
    entry_index  BIGINT    NOT NULL,
    entry        BYTEA,
    is_full_slot BOOL      NOT NULL,
    updated_on   TIMESTAMP NOT NULL
);

CREATE INDEX index_untrusted_entry_slot_entry_index ON untrusted_entry (slot, entry_index);
CREATE INDEX index_untrusted_entry_slot ON untrusted_entry (slot);
CREATE INDEX index_untrusted_entry_parent_slot ON untrusted_entry (parent_slot);

/**
 * The following is for keeping historical data for accounts and is not required for plugin to work.
 */
-- The table storing historical data for accounts
CREATE TABLE account_audit
(
    id            bigserial PRIMARY KEY,
    pubkey        BYTEA     NOT NULL,
    owner         BYTEA,
    lamports      BIGINT    NOT NULL,
    slot          BIGINT    NOT NULL,
    executable    BOOL      NOT NULL,
    rent_epoch    BIGINT    NOT NULL,
    data          BYTEA,
    write_version BIGINT    NOT NULL,
    txn_signature BYTEA,
    updated_on    TIMESTAMP NOT NULL
);

CREATE INDEX index_account_audit_pubkey_write_version ON account_audit (pubkey, write_version);

CREATE INDEX index_account_audit_pubkey_slot ON account_audit (pubkey, slot);

CREATE INDEX index_account_audit_pubkey_owner ON account_audit (pubkey, owner);

CREATE INDEX index_account_audit_slot ON account_audit (slot);


CREATE FUNCTION audit_account_update() RETURNS trigger AS $audit_account_update$
    BEGIN
		INSERT INTO account_audit (pubkey, owner, lamports, slot, executable,
		                           rent_epoch, data, write_version, updated_on, txn_signature)
            VALUES (OLD.pubkey, OLD.owner, OLD.lamports, OLD.slot,
                    OLD.executable, OLD.rent_epoch, OLD.data,
                    OLD.write_version, OLD.updated_on, OLD.txn_signature);
        RETURN NEW;
    END;

$audit_account_update$ LANGUAGE plpgsql;

CREATE TRIGGER audit_account_update_trigger AFTER UPDATE OR DELETE ON account
    FOR EACH ROW EXECUTE PROCEDURE audit_account_update();


CREATE TABLE account_inspect (
    id            bigserial PRIMARY KEY,
    pubkey        BYTEA     NOT NULL,
    owner         BYTEA,
    lamports      BIGINT    NOT NULL,
    slot          BIGINT    NOT NULL,
    executable    BOOL      NOT NULL,
    rent_epoch    BIGINT    NOT NULL,
    data          BYTEA,
    write_version BIGINT    NOT NULL,
    txn_signature BYTEA,
    updated_on    TIMESTAMP NOT NULL
);


CREATE INDEX index_account_inspect_pubkey_write_version ON account_inspect (pubkey, write_version);

CREATE INDEX index_account_inspect_pubkey_slot ON account_inspect (pubkey, slot);

CREATE INDEX index_account_inspect_pubkey_owner ON account_inspect (pubkey, owner);

CREATE INDEX index_account_inspect_slot ON account_inspect (slot);



CREATE FUNCTION inspect_account_update() RETURNS trigger AS $inspect_account_update$
    BEGIN
		INSERT INTO account_inspect (pubkey, owner, lamports, slot, executable,
		                           rent_epoch, data, write_version, updated_on, txn_signature)
            VALUES (NEW.pubkey, NEW.owner, NEW.lamports, NEW.slot,
                    NEW.executable, NEW.rent_epoch, NEW.data,
                    NEW.write_version, NEW.updated_on, NEW.txn_signature);
        RETURN NEW;
    END;

$inspect_account_update$ LANGUAGE plpgsql;

CREATE TRIGGER inspect_account_update_trigger AFTER INSERT OR UPDATE OR DELETE ON account
    FOR EACH ROW EXECUTE PROCEDURE inspect_account_update();

