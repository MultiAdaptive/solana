/**
 * Script for cleaning up the schema for PostgreSQL used for the AccountsDb plugin.
 */

DROP TRIGGER account_modify_trigger ON account;
DROP FUNCTION account_modify;
DROP TABLE account_audit;
DROP TABLE account CASCADE;
DROP TABLE slot;
DROP TABLE transaction;
DROP TABLE block;
DROP TABLE spl_token_owner_index;
DROP TABLE spl_token_mint_index;
DROP TABLE untrusted_entry;
DROP TABLE merkle_tree_proof;
DROP TABLE genesis;
DROP TABLE brief;

DROP TYPE "TransactionStatusMeta" CASCADE;
DROP TYPE "TransactionError" CASCADE;
DROP TYPE "TransactionErrorCode" CASCADE;
DROP TYPE "LoadedMessageV0" CASCADE;
DROP TYPE "LoadedAddresses" CASCADE;
DROP TYPE "TransactionMessageV0" CASCADE;
DROP TYPE "TransactionMessage" CASCADE;
DROP TYPE "TransactionMessageHeader" CASCADE;
DROP TYPE "TransactionMessageAddressTableLookup" CASCADE;
DROP TYPE "Reward" CASCADE;
DROP TYPE "RewardType" CASCADE;
DROP TYPE "TransactionTokenBalance" CASCADE;
DROP TYPE "InnerInstructions" CASCADE;
DROP TYPE "CompiledInstruction" CASCADE;
