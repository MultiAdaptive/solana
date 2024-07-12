use solana_sdk::hash::Hash;
use solana_sdk::instruction::CompiledInstruction;
use solana_sdk::message::{Message, MessageHeader};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::Transaction;

use crate::entities::sql_types::DbTransactionMessage;

pub fn construct_tx(msg: DbTransactionMessage, signatures: Vec<Vec<u8>>) -> Transaction {
    Transaction {
        signatures: signatures.into_iter().map(|s| Signature::try_from(s.as_slice()).unwrap()).collect(),
        message: Message {
            header: MessageHeader {
                num_required_signatures: msg.header.num_required_signatures as u8,
                num_readonly_signed_accounts: msg.header.num_readonly_signed_accounts as u8,
                num_readonly_unsigned_accounts: msg.header.num_readonly_unsigned_accounts as u8,
            },
            account_keys: msg.account_keys.iter().map(|ak| Pubkey::try_from(ak.as_slice()).unwrap()).collect(),
            recent_blockhash: Hash::new(&msg.recent_blockhash),
            instructions: msg
                .instructions
                .iter()
                .map(|i| {
                    CompiledInstruction::new_from_raw_parts(
                        i.program_id_index as u8,
                        i.data.clone(),
                        i.accounts.iter().map(|a| *a as u8).collect(),
                    )
                })
                .collect(),
        },
    }
}
