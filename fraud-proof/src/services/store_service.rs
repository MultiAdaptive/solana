use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, RwLock};

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel::row::NamedRow;
use itertools::Itertools;
use log::{error, info};
use postgres::Client;
use solana_sdk::account::Account;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::Transaction;
use sparse_merkle_tree::blake2b::Blake2bHasher;
use sparse_merkle_tree::default_store::DefaultStore;
use sparse_merkle_tree::SparseMerkleTree;
use sparse_merkle_tree::traits::Value;

use crate::common::node_error::NodeError;
use crate::entities::account_audit_entity::table_account_audit::dsl::{
    column_slot as column_aa_slot, column_write_version, table_account_audit,
};
use crate::entities::slot_entity::table_slot::dsl::{
    column_slot as column_s_slot, table_slot,
};
use crate::entities::sql_types::DbTransactionMessage;
use crate::fraud_proof::chain_brief::ChainBrief;
use crate::fraud_proof::wrap_key::WrapKey;
use crate::fraud_proof::wrap_slot::WrapSlot;
use crate::model::account_audit_row::AccountAuditRow;
use crate::model::hash_record::HashRecord;
use crate::model::slot_row::SlotRow;
use crate::model::slot_summary::SlotSummary;
use crate::model::smt_account::{MemoryStoreSMT, SMTAccount};
use crate::utils::account_util::compute_ha;
use crate::utils::store_util::{PgConnectionPool, PooledPgConnection};
use crate::utils::time_util;
use crate::utils::tx_util::construct_tx;
use crate::utils::uuid_util::generate_uuid;

const TX_STMT: &str =
    "SELECT legacy_message, signatures FROM transaction WHERE slot = $1 ORDER BY write_version ASC";
const MAX_SLOT_STMT: &str = "SELECT MAX(slot) FROM block";

pub struct StoreService<'a> {
    pub client_pool: &'a PgConnectionPool,
    pub client_one: &'a mut Client,
}

impl StoreService<'_> {
    pub fn generate_until_briefs(&mut self, wrap_slot: WrapSlot) -> Result<Vec<ChainBrief>, NodeError> {
        let current_slot: u64 = wrap_slot.slot.clone();

        // The slot 0 and slot 1 are initial of blockchain, we never challenge, so skip them.
        let briefs: Vec<ChainBrief> = self.generate_range_briefs(2, current_slot.clone()).unwrap();

        return Ok(briefs);
    }

    pub fn generate_range_briefs(&mut self, start_slot: u64, end_slot: u64) -> Result<Vec<ChainBrief>, NodeError> {
        let slot_summary: HashMap<u64, SlotSummary> = self.generate_until_slot_summary(end_slot.clone()).unwrap();

        let briefs: Vec<ChainBrief> = self.generate_range_briefs_from_slot_summary(start_slot.clone(), end_slot.clone(), slot_summary.clone()).unwrap();

        return Ok(briefs);
    }

    pub fn generate_range_briefs_from_slot_summary(&mut self, start_slot: u64, end_slot: u64, slot_summary: HashMap<u64, SlotSummary>) -> Result<Vec<ChainBrief>, NodeError> {
        let mut briefs: Vec<ChainBrief> = vec![];
        // The slot 0 and slot 1 are initial of blockchain, we never challenge, so skip them.
        for i in start_slot.clone()..=end_slot.clone() {
            let slot = i;
            //The root_hash of the current slot is the second_hash of the next slot
            let root_hash_str: String = slot_summary.get(&(slot.clone() + 1)).cloned().unwrap().record.second_hash;
            let root_hash: Hash = Hash::from_str(root_hash_str.as_str()).unwrap();
            //The third_hash is the hash_account of the current slot
            let hash_account_str: String = slot_summary.get(&(slot.clone())).cloned().unwrap().record.third_hash;
            let hash_account: Hash = Hash::from_str(hash_account_str.as_str()).unwrap();
            let transaction_number: u32 = slot_summary.get(&(slot.clone())).cloned().unwrap().count as u32;

            let brief = ChainBrief {
                slot: slot,
                root_hash: root_hash_str,
                hash_account: hash_account_str,
                transaction_number: transaction_number,
            };
            briefs.push(brief);
        }

        return Ok(briefs);
    }

    fn generate_until_slot_summary(&mut self, end_slot: u64) -> Result<HashMap<u64, SlotSummary>, NodeError> {
        let smt_tree = MemoryStoreSMT::new_with_store(Default::default());
        let smt = Arc::new(RwLock::new(smt_tree.unwrap()));

        let conn: &mut PooledPgConnection = &mut self.client_pool.get()?;

        let mut slot_summary: HashMap<u64, SlotSummary> = HashMap::new();
        let mut cur_slot = 0;

        while cur_slot <= end_slot.clone() + 1 {
            // 1. Query `account_audit` by `slot` ordered by `write_version`.
            let results = table_account_audit
                .filter(column_aa_slot.eq(cur_slot.clone() as i64))
                .order(column_write_version.asc())
                .load::<AccountAuditRow>(conn)
                .expect("Error loading account_audit");

            // 2. If results is empty, check if we hit the top of slot
            if results.is_empty() {
                let results: Vec<SlotRow> = table_slot
                    .order(column_s_slot.desc())
                    .limit(1)
                    .load::<SlotRow>(conn)
                    .expect("Error loading slot");
                if results.is_empty() {
                    info!("No data in PG, SMT start too early.");
                    break;
                }

                if results[0].slot as u64 > cur_slot {
                    cur_slot += 1;
                } else {
                    // yeah, we hit the top of data
                    time_util::sleep_seconds(1);
                }
                continue;
            }

            // If results exists, create SMT with results.
            let mut tx_found = false;
            let mut second_hash = String::new();
            results.iter().for_each(|aa| {
                if aa.txn_signature.is_some() && !tx_found {
                    tx_found = true;
                    second_hash =
                        Hash::new(smt.read().unwrap().root().as_slice().clone()).to_string();
                }

                let raw_acct = SMTAccount {
                    pubkey: Pubkey::try_from(aa.pubkey.as_slice()).unwrap(),
                    lamports: aa.lamports,
                    owner: Pubkey::try_from(aa.owner.clone().unwrap().as_slice()).unwrap(),
                    executable: aa.executable,
                    rent_epoch: aa.rent_epoch,
                    data: aa.data.clone().unwrap(),
                };
                let key_hash = raw_acct.smt_key();
                if let Err(_err) = smt.write().unwrap().update(key_hash, raw_acct) {
                    error!("Update SMT key_value failed with err: {}", _err);
                    return;
                }
            });

            // Save root
            // The slot 0 and slot 1 are initial of blockchain, we never challenge, so skip them.
            if cur_slot >= 2 {
                let smt_root =
                    Hash::new(smt.read().unwrap().root().as_slice().clone()).to_string();

                // info!("###################### {:?}, {:?}", Hash::new(smt.read().unwrap().root().as_slice().clone()), Hash::new(smt.read().unwrap().root().as_slice().clone()).to_string());
                let last_slot = cur_slot.clone() - 1;
                //The first_hash of the current slot is the fourth_hash of the previous slot
                let first_hash = match slot_summary.get(&last_slot) {
                    Some(ss) => ss.record.fourth_hash.clone(),
                    _ => String::new(),
                };
                slot_summary
                    .entry(cur_slot.clone())
                    .and_modify(|ss| {
                        ss.record.first_hash = first_hash.clone();
                        ss.record.second_hash = second_hash.clone();
                        ss.record.fourth_hash = smt_root.clone()
                    })
                    .or_insert(SlotSummary {
                        slot: cur_slot.clone(),
                        record: HashRecord {
                            first_hash,
                            second_hash,
                            third_hash: String::new(),
                            fourth_hash: smt_root,
                        },
                        ..Default::default()
                    });
            }

            // 4. next slot
            cur_slot += 1;
        }

        let tx_stmt = self.client_one.prepare(TX_STMT).unwrap();

        for (k, v) in slot_summary.iter_mut().sorted_by_key(|x| x.0) {
            let tx_results = self.client_one.query(&tx_stmt, &[&(*k as i64)]).unwrap();
            let aas = self.query_aas_by_slot(*k)?;

            let mut ha = Hash::default();
            for r in &tx_results {
                let msg: DbTransactionMessage = r.get(0);
                let signatures: Vec<Vec<u8>> = r.get(1);
                let pks: Vec<Pubkey> = msg.account_keys.iter().map(|ak| Pubkey::try_from(ak.as_slice()).unwrap()).collect();

                // Found all related (modified) accounts from account_audit
                let modified_accounts: Vec<AccountAuditRow> = aas
                    .iter()
                    .filter(|a| pks.contains(&Pubkey::try_from(a.pubkey.as_slice()).unwrap()) && a.txn_signature.is_some())
                    .cloned()
                    .collect();

                ha = compute_ha(
                    &Signature::try_from(signatures[0].as_slice()).unwrap(),
                    &modified_accounts
                        .iter()
                        .map(|a| a.to_smt_account())
                        .collect::<Vec<SMTAccount>>(),
                    &ha,
                );
            }

            v.record.third_hash = ha.to_string();
            v.count = tx_results.len();
        }

        return Ok(slot_summary);
    }


    fn generate_slot_smt(&mut self, end_slot: u64) -> Result<SparseMerkleTree<Blake2bHasher, SMTAccount, DefaultStore<SMTAccount>>, NodeError> {
        let smt_tree = MemoryStoreSMT::new_with_store(Default::default());
        let mut smt = smt_tree.unwrap();

        let conn: &mut PooledPgConnection = &mut self.client_pool.get()?;

        let mut cur_slot = 0;

        while cur_slot <= end_slot.clone() + 1 {
            // 1. Query `account_audit` by `slot` ordered by `write_version`.
            let results = table_account_audit
                .filter(column_aa_slot.eq(cur_slot.clone() as i64))
                .order(column_write_version.asc())
                .load::<AccountAuditRow>(conn)
                .expect("Error loading account_audit");

            // 2. If results is empty, check if we hit the top of slot
            if results.is_empty() {
                let results: Vec<SlotRow> = table_slot
                    .order(column_s_slot.desc())
                    .limit(1)
                    .load::<SlotRow>(conn)
                    .expect("Error loading slot");
                if results.is_empty() {
                    info!("No data in PG, SMT start too early.");
                    break;
                }

                if results[0].slot as u64 > cur_slot {
                    cur_slot += 1;
                } else {
                    // yeah, we hit the top of data
                    time_util::sleep_seconds(1);
                }
                continue;
            }

            // If results exists, create SMT with results.
            let mut tx_found = false;
            results.iter().for_each(|aa| {
                if aa.txn_signature.is_some() && !tx_found {
                    tx_found = true;
                }

                let raw_acct = SMTAccount {
                    pubkey: Pubkey::try_from(aa.pubkey.as_slice()).unwrap(),
                    lamports: aa.lamports,
                    owner: Pubkey::try_from(aa.owner.clone().unwrap().as_slice()).unwrap(),
                    executable: aa.executable,
                    rent_epoch: aa.rent_epoch,
                    data: aa.data.clone().unwrap(),
                };
                let key_hash = raw_acct.smt_key();
                if let Err(_err) = smt.update(key_hash, raw_acct) {
                    error!("Update SMT key_value failed with err: {}", _err);
                    return;
                }
            });

            // 4. next slot
            cur_slot += 1;
        }

        Ok(smt)
    }

    pub fn generate_initial_slot_summary_and_smt(
        &mut self,
        end_slot: u64,
    ) -> Result<(SparseMerkleTree<Blake2bHasher, SMTAccount, DefaultStore<SMTAccount>>, HashMap<u64, SlotSummary>), NodeError> {
        let smt_tree = MemoryStoreSMT::new_with_store(Default::default());
        let mut smt = smt_tree.unwrap();

        let conn: &mut PooledPgConnection = &mut self.client_pool.get()?;

        let mut slot_summary: HashMap<u64, SlotSummary> = HashMap::new();
        let mut cur_slot = 0;

        while cur_slot <= end_slot.clone() + 1 {
            // 1. Query `account_audit` by `slot` ordered by `write_version`.
            let results = table_account_audit
                .filter(column_aa_slot.eq(cur_slot.clone() as i64))
                .order(column_write_version.asc())
                .load::<AccountAuditRow>(conn)
                .expect("Error loading account_audit");

            // 2. If results is empty, check if we hit the top of slot
            if results.is_empty() {
                let results: Vec<SlotRow> = table_slot
                    .order(column_s_slot.desc())
                    .limit(1)
                    .load::<SlotRow>(conn)
                    .expect("Error loading slot");
                if results.is_empty() {
                    info!("No data in PG, SMT start too early.");
                    break;
                }

                if results[0].slot as u64 > cur_slot {
                    cur_slot += 1;
                } else {
                    // yeah, we hit the top of data
                    time_util::sleep_seconds(1);
                }
                continue;
            }

            // If results exists, create SMT with results.
            let mut tx_found = false;
            let mut second_hash = String::new();
            results.iter().for_each(|aa| {
                if aa.txn_signature.is_some() && !tx_found {
                    tx_found = true;
                    second_hash = Hash::new(smt.root().as_slice().clone()).to_string();
                }

                let raw_acct = SMTAccount {
                    pubkey: Pubkey::try_from(aa.pubkey.as_slice()).unwrap(),
                    lamports: aa.lamports,
                    owner: Pubkey::try_from(aa.owner.clone().unwrap().as_slice()).unwrap(),
                    executable: aa.executable,
                    rent_epoch: aa.rent_epoch,
                    data: aa.data.clone().unwrap(),
                };
                let key_hash = raw_acct.smt_key();
                if let Err(_err) = smt.update(key_hash, raw_acct) {
                    error!("Update SMT key_value failed with err: {}", _err);
                    return;
                }
            });

            // Save root
            if cur_slot >= 2 {
                let smt_root = Hash::new(smt.root().as_slice().clone()).to_string();
                let last_slot = cur_slot.clone() - 1;
                let first_hash = match slot_summary.get(&last_slot) {
                    Some(ss) => ss.record.fourth_hash.clone(),
                    _ => String::new(),
                };
                slot_summary
                    .entry(cur_slot.clone())
                    .and_modify(|ss| {
                        ss.record.first_hash = first_hash.clone();
                        ss.record.second_hash = second_hash.clone();
                        ss.record.fourth_hash = smt_root.clone();
                    })
                    .or_insert(SlotSummary {
                        slot: cur_slot.clone(),
                        record: HashRecord {
                            first_hash,
                            second_hash,
                            third_hash: String::new(),
                            fourth_hash: smt_root,
                        },
                        ..Default::default()
                    });
            }

            // 4. next slot
            cur_slot += 1;
        }

        let tx_stmt = self.client_one.prepare(TX_STMT).unwrap();

        for (k, v) in slot_summary.iter_mut().sorted_by_key(|x| x.0) {
            let tx_results = self.client_one.query(&tx_stmt, &[&(*k as i64)]).unwrap();
            let aas = self.query_aas_by_slot(*k)?;

            let mut ha = Hash::default();
            for r in &tx_results {
                let msg: DbTransactionMessage = r.get(0);
                let signatures: Vec<Vec<u8>> = r.get(1);
                let pks: Vec<Pubkey> = msg.account_keys.iter().map(|ak| Pubkey::try_from(ak.as_slice()).unwrap()).collect();

                // Found all related (modified) accounts from account_audit
                let modified_accounts: Vec<AccountAuditRow> = aas
                    .iter()
                    .filter(|a| pks.contains(&Pubkey::try_from(a.pubkey.as_slice()).unwrap()) && a.txn_signature.is_some())
                    .cloned()
                    .collect();

                ha = compute_ha(
                    &Signature::try_from(signatures[0].as_slice()).unwrap(),
                    &modified_accounts
                        .iter()
                        .map(|a| a.to_smt_account())
                        .collect::<Vec<SMTAccount>>(),
                    &ha,
                );
            }

            v.record.third_hash = ha.to_string();
            v.count = tx_results.len();
        }

        Ok((smt, slot_summary))
    }

    pub fn generate_continue_slot_summary_and_smt(
        &mut self,
        start_slot: u64,
        end_slot: u64,
        mut smt_tree: SparseMerkleTree<Blake2bHasher, SMTAccount, DefaultStore<SMTAccount>>,
        mut slot_summary: HashMap<u64, SlotSummary>,
    ) -> Result<(SparseMerkleTree<Blake2bHasher, SMTAccount, DefaultStore<SMTAccount>>, HashMap<u64, SlotSummary>), NodeError> {
        let conn: &mut PooledPgConnection = &mut self.client_pool.get()?;

        let mut cur_slot = start_slot;

        while cur_slot <= end_slot + 1 {
            // 1. Query `account_audit` by `slot` ordered by `write_version`.
            let results = table_account_audit
                .filter(column_aa_slot.eq(cur_slot as i64))
                .order(column_write_version.asc())
                .load::<AccountAuditRow>(conn)
                .expect("Error loading account_audit");

            // 2. If results is empty, check if we hit the top of slot
            if results.is_empty() {
                let results: Vec<SlotRow> = table_slot
                    .order(column_s_slot.desc())
                    .limit(1)
                    .load::<SlotRow>(conn)
                    .expect("Error loading slot");
                if results.is_empty() {
                    info!("No data in PG, SMT start too early.");
                    break;
                }

                if results[0].slot as u64 > cur_slot {
                    cur_slot += 1;
                } else {
                    // yeah, we hit the top of data
                    time_util::sleep_seconds(1);
                }
                continue;
            }

            // If results exists, create SMT with results.
            let mut tx_found = false;
            let mut second_hash = String::new();
            results.iter().for_each(|aa| {
                if aa.txn_signature.is_some() && !tx_found {
                    tx_found = true;
                    second_hash = Hash::new(smt_tree.root().as_slice().clone()).to_string();
                }

                let raw_acct = SMTAccount {
                    pubkey: Pubkey::try_from(aa.pubkey.as_slice()).unwrap(),
                    lamports: aa.lamports,
                    owner: Pubkey::try_from(aa.owner.clone().unwrap().as_slice()).unwrap(),
                    executable: aa.executable,
                    rent_epoch: aa.rent_epoch,
                    data: aa.data.clone().unwrap(),
                };
                let key_hash = raw_acct.smt_key();
                if let Err(_err) = smt_tree.update(key_hash, raw_acct) {
                    error!("Update SMT key_value failed with err: {}", _err);
                    return;
                }
            });

            // Save root
            if cur_slot >= 2 {
                let smt_root = Hash::new(smt_tree.root().as_slice().clone()).to_string();
                let last_slot = cur_slot - 1;
                let first_hash = match slot_summary.get(&last_slot) {
                    Some(ss) => ss.record.fourth_hash.clone(),
                    _ => String::new(),
                };
                slot_summary
                    .entry(cur_slot)
                    .and_modify(|ss| {
                        ss.record.first_hash = first_hash.clone();
                        ss.record.second_hash = second_hash.clone();
                        ss.record.fourth_hash = smt_root.clone();
                    })
                    .or_insert(SlotSummary {
                        slot: cur_slot,
                        record: HashRecord {
                            first_hash,
                            second_hash,
                            third_hash: String::new(),
                            fourth_hash: smt_root,
                        },
                        ..Default::default()
                    });
            }

            // 4. next slot
            cur_slot += 1;
        }

        let tx_stmt = self.client_one.prepare(TX_STMT).unwrap();

        for (k, v) in slot_summary.iter_mut().sorted_by_key(|x| x.0) {
            if *k < start_slot {
                continue;
            }
            let tx_results = self.client_one.query(&tx_stmt, &[&(*k as i64)]).unwrap();
            let aas = self.query_aas_by_slot(*k)?;

            let mut ha = Hash::default();
            for r in &tx_results {
                let msg: DbTransactionMessage = r.get(0);
                let signatures: Vec<Vec<u8>> = r.get(1);
                let pks: Vec<Pubkey> = msg.account_keys.iter().map(|ak| Pubkey::try_from(ak.as_slice()).unwrap()).collect();

                // Found all related (modified) accounts from account_audit
                let modified_accounts: Vec<AccountAuditRow> = aas
                    .iter()
                    .filter(|a| pks.contains(&Pubkey::try_from(a.pubkey.as_slice()).unwrap()) && a.txn_signature.is_some())
                    .cloned()
                    .collect();

                ha = compute_ha(
                    &Signature::try_from(signatures[0].as_slice()).unwrap(),
                    &modified_accounts
                        .iter()
                        .map(|a| a.to_smt_account())
                        .collect::<Vec<SMTAccount>>(),
                    &ha,
                );
            }

            v.record.third_hash = ha.to_string();
            v.count = tx_results.len();
        }

        Ok((smt_tree, slot_summary))
    }

    // SELECT pubkey, owner, lamports, slot, executable, rent_epoch, data, write_version, txn_signature FROM account_audit WHERE slot = $1 ORDER BY write_version ASC;
    fn query_aas_by_slot(&mut self, query_slot: u64) -> Result<Vec<AccountAuditRow>, NodeError> {
        let conn: &mut PooledPgConnection = &mut self.client_pool.get()?;

        // Query modified accounts from account_audit table.
        let results = table_account_audit
            .filter(column_aa_slot.eq(query_slot as i64))
            .order(column_write_version.asc())
            .load::<AccountAuditRow>(conn)
            .expect("Error loading account_audit");
        if results.is_empty() {
            return Err(
                NodeError::new(generate_uuid(),
                               format!("Couldn't find query slot {}'s account_audit", query_slot.clone()),
                )
            );
        }

        Ok(results)
    }

    pub fn query_max_slot_from_block(&mut self) -> Result<u64, NodeError> {
        let max_slot_stmt = self.client_one.prepare(MAX_SLOT_STMT).unwrap();
        let results = self.client_one.query(&max_slot_stmt, &[]).unwrap();
        if results.is_empty() {
            return Err(
                NodeError::new(generate_uuid(),
                               format!("Couldn't find query max slot from block"),
                )
            );
        }
        let max_slot: i64 = results[0].get(0);

        Ok(max_slot as u64)
    }
}

