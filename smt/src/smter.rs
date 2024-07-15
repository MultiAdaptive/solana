use blake2b_rs::Blake2bBuilder;
use solana_sdk::signature::Signature;
use sparse_merkle_tree::traits::Value;
use std::collections::HashMap;
use std::str::FromStr;

use {
    crate::{
        account_smt::{RocksStoreSMT, SMTAccount},
        model::*,
        rocks_store::RocksStore,
    },
    diesel::{prelude::*, Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl},
    itertools::Itertools,
    log::{error, info},
    postgres::{Client, NoTls},
    rocksdb::*,
    solana_sdk::{
        hash::Hash,
        instruction::CompiledInstruction,
        message::{Message, MessageHeader},
        pubkey::Pubkey,
        transaction::Transaction,
    },
    std::{
        fs, io,
        path::{Path, PathBuf},
        sync::{Arc, RwLock},
    },
    tempfile::TempDir,
};
use crate::merkle_verify::MerkleVerify;

const SMT_SUBDIR: &str = "smt";
const RECORD_SUBDIR: &str = "record";
const AA_STMT: &str = "SELECT pubkey, owner, lamports, slot, executable, rent_epoch, data, write_version, txn_signature FROM account_audit WHERE slot = $1 ORDER BY write_version ASC";
const TX_STMT: &str =
    "SELECT legacy_message, signatures FROM transaction WHERE slot = $1 ORDER BY write_version ASC";
const BLOCK_STMT: &str = "SELECT block_height FROM block WHERE slot = $1";

pub struct SMTer {
    client: PgConnection,
    smt: Arc<RwLock<RocksStoreSMT>>,
    util_db: DB,
    pub start_slot: u64, // last slot
}

impl SMTer {
    pub fn new(config: &PostgresConfig, ledger_path: &PathBuf) -> Result<Self, SMTerError> {
        let client = Self::connect_to_db(config)?;

        let smt_path = ledger_path.as_path().join(SMT_SUBDIR);
        let smt = Self::start_rocks_smt(&smt_path)?;

        let util_path = ledger_path.as_path().join(RECORD_SUBDIR);
        let (util_db, start_slot) = Self::start_rocks_record(&util_path)?;

        Ok(Self {
            client,
            smt,
            util_db,
            start_slot,
        })
    }

    fn start_rocks_smt(smt_path: &PathBuf) -> Result<Arc<RwLock<RocksStoreSMT>>, SMTerError> {
        let db = DB::open_default(smt_path);
        if db.is_err() {
            return Err(SMTerError::InitSMTDBError {
                msg: "Failed to open SMT rocksdb.".to_string(),
            });
        }

        let rocksdb_store = RocksStore::new(db.unwrap());
        let smt_tree = RocksStoreSMT::new_with_store(rocksdb_store);
        if smt_tree.is_err() {
            return Err(SMTerError::InitSMTError {
                msg: "Failed to new store for SMT".to_string(),
            });
        }

        Ok(Arc::new(RwLock::new(smt_tree.unwrap())))
    }

    fn start_rocks_record(util_path: &PathBuf) -> Result<(DB, u64), SMTerError> {
        let util_db = DB::open_default(util_path);
        if util_db.is_err() {
            return Err(SMTerError::InitRecordDBError {
                msg: "Failed to open Record rocksdb.".to_string(),
            });
        }

        let mut start_slot = 0_u64;
        if let Ok(value) = util_db.as_ref().unwrap().get("slot") {
            if let Some(v) = value {
                let v: [u8; 8] = v.try_into().unwrap_or_else(|v: Vec<u8>| {
                    panic!("Expected a Vec of length {} but it was {}", 8, v.len())
                });
                start_slot = u64::from_le_bytes(v);
            }
        }

        Ok((util_db.unwrap(), start_slot))
    }

    fn connect_to_db(config: &PostgresConfig) -> Result<PgConnection, SMTerError> {
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.user.as_ref().unwrap(),
            config.password.as_ref().unwrap(),
            config.host.as_ref().unwrap(),
            config.port.as_ref().unwrap(),
            config.dbname.as_ref().unwrap(),
        );

        let pg_connection = PgConnection::establish(&database_url);
        if pg_connection.is_err() {
            return Err(SMTerError::DbConnectError {
                msg: format!("Error connecting to {}", database_url),
            });
        }

        Ok(pg_connection.unwrap())
    }

    fn connect_to_db_raw(config: &PostgresConfig) -> Result<Client, SMTerError> {
        let connection_str = format!(
            "host={} user={} password={} dbname={} port={}",
            config.host.as_ref().unwrap(),
            config.user.as_ref().unwrap(),
            config.password.as_ref().unwrap(),
            config.dbname.as_ref().unwrap(),
            config.port.as_ref().unwrap(),
        );

        let client =
            Client::connect(&connection_str, NoTls).map_err(|_| SMTerError::DbConnectError {
                msg: format!("the config is {}", connection_str),
            })?;

        Ok(client)
    }

    pub fn update_smt(
        config: &PostgresConfig,
        ledger_path: &PathBuf,
        end_slot: u64,
    ) -> Result<Vec<SlotSummary>, SMTerError> {
        let mut smter = Self::new(&config, &ledger_path).unwrap();
        let mut summary = smter.fetch_db(end_slot).unwrap();
        let _ = Self::calculate_has(&config, &mut summary);
        Ok(summary.values().cloned().collect())
    }

    fn fetch_db(&mut self, end_slot: u64) -> Result<HashMap<u64, SlotSummary>, SMTerError> {
        use crate::smter::table_account_audit::column_write_version;
        use crate::smter::table_account_audit::dsl::{
            column_slot as column_aa_slot, table_account_audit,
        };
        use crate::smter::table_slot::dsl::{column_slot, table_slot};

        if self.start_slot == end_slot {
            return Err(SMTerError::UpdatingSMTError {
                msg: format!("such slot already updated!"),
            });
        }

        let mut slot_summary: HashMap<u64, SlotSummary> = HashMap::new();
        let mut cur_slot = self.start_slot;
        while cur_slot < end_slot + 1 {
            // 1. Query `account_audit` by `slot` ordered by `write_version`.
            let results: Vec<AccountAuditRow> = table_account_audit
                .filter(column_aa_slot.eq(cur_slot as i64))
                .order(column_write_version.asc())
                .load::<AccountAuditRow>(&mut self.client)
                .expect("Error loading account_audit");

            // 2. If results is empty, check if we hit the top of slot
            if results.is_empty() {
                let results: Vec<SlotRow> = table_slot
                    .order(column_slot.desc())
                    .limit(1)
                    .load::<SlotRow>(&mut self.client)
                    .expect("Error loading slot");
                if results.is_empty() {
                    info!("No data in PG, SMT start too early.");
                    break;
                }

                if results[0].slot as u64 > cur_slot {
                    cur_slot += 1;
                } else {
                    // yeah, we hit the top of data
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
                continue;
            }

            // If results exists, update SMT with results.
            let mut tx_found = false;
            let mut second_hash = String::new();
            results.iter().for_each(|aa| {
                if aa.txn_signature.is_some() && !tx_found {
                    tx_found = true;
                    second_hash =
                        Hash::new(self.smt.read().unwrap().root().as_slice().clone()).to_string();
                }

                let raw_acct = SMTAccount {
                    pubkey: Pubkey::try_from(aa.pubkey.as_slice()).unwrap(),
                    lamports: aa.lamports,
                    owner: Pubkey::try_from(aa.owner.as_slice()).unwrap(),
                    executable: aa.executable,
                    rent_epoch: aa.rent_epoch,
                    data: aa.data.clone(),
                };
                let key_hash = raw_acct.smt_key();
                if let Err(_err) = self.smt.write().unwrap().update(key_hash, raw_acct) {
                    error!("Update SMT key_value failed with err: {}", _err);
                    return;
                }
            });

            // 3. Update local record
            let v = cur_slot.to_le_bytes();
            if let Err(_err) = self.util_db.put("slot", v) {
                error!(
                    "Update local record rocksdb failed with err: {}",
                    _err.into_string()
                );
                break;
            }

            // Save root
            // The slot 0 and slot 1 are initial of blockchain, we never challenge, so skip them.
            if cur_slot >= 2 {
                let smt_root =
                    Hash::new(self.smt.read().unwrap().root().as_slice().clone()).to_string();
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
                        ss.record.fourth_hash = smt_root.clone()
                    })
                    .or_insert(SlotSummary {
                        slot: cur_slot,
                        record: HashRecord {
                            first_hash,
                            second_hash,
                            thrid_hash: String::default(),
                            fourth_hash: smt_root,
                        },
                        ..Default::default()
                    });
            }

            // 4. next slot
            cur_slot += 1;
        }

        Ok(slot_summary)
    }

    fn calculate_has(
        config: &PostgresConfig,
        summary: &mut HashMap<u64, SlotSummary>,
    ) -> Result<(), SMTerError> {
        let mut client = Self::connect_to_db_raw(&config)?;
        let tx_stmt = client.prepare(TX_STMT).unwrap();

        for (k, v) in summary.iter_mut().sorted_by_key(|x| x.0) {
            let tx_results = client.query(&tx_stmt, &[&(*k as i64)]).unwrap();
            let aas = Self::query_aas_by_slot(&mut client, *k)?;

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

            v.record.thrid_hash = ha.to_string();
            v.count = tx_results.len();
        }

        Ok(())
    }

    /*
            start of `t + 1`
                 ↓
      ...|SLOT t| - - - - - - - - - - |SLOT t+1|...
                ↑                              ↑
        root point of `t`                 root point of `t+1`
    */
    // When we query slot `t+1`'s `tx_id` transaction, it means
    // 1. the slot `t`'s SMT is calculated and persisted
    // 2. we calculate temporary SMT based on t ordered by write_version, and reach tx_id transaction
    // 3. Return the pre-changed accounts and post-changed accounts in tx_id transaction, and SMT root too.
    // 4. delete the temporary SMT directory.
    pub fn query_tx_context(
        config: &PostgresConfig,
        ledger_path: &PathBuf,
        query_slot: u64,
        tx_no: usize,
    ) -> Result<TxContext, SMTerError> {
        // Check local record to see if the query slot is satisfied.
        let util_path = ledger_path.as_path().join(RECORD_SUBDIR);
        let (_, start_slot) = Self::start_rocks_record(&util_path)?;
        if start_slot + 1 != query_slot {
            return Err(SMTerError::QuerySlotNotMatchError {
                msg: format!("local_slot is {}, while your query_slot is {}, you should update or re-construct SMT to make sure that `local_slot + 1 = query_slot`", start_slot, query_slot)
            });
        }

        // Create a temp dir, copy all exists SMT rocks data
        let temp_dir = TempDir::new().unwrap();
        let smt_path = ledger_path.as_path().join(SMT_SUBDIR);
        if let Err(err) = copy_dir_all(smt_path.as_path(), temp_dir.as_ref()) {
            return Err(SMTerError::QueryCopySMTError {
                msg: err.to_string(),
            });
        }

        // Start SMT with temp rocks data.
        let smt = Self::start_rocks_smt(&temp_dir.path().to_path_buf())?;

        // Query PG.
        let mut client = Self::connect_to_db_raw(&config)?;
        let block_height = Self::query_blockheight_by_slot(&mut client, query_slot)?;

        // Query transactions from transaction table at slot query_slot.
        let txs = Self::query_txs_by_slot(&mut client, query_slot)?;
        if txs.len() <= tx_no {
            return Err(SMTerError::QueryTxNotExistsError {
                msg: format!(
                    "Transaction index does not exist! Only {} tx in the slot!",
                    txs.len()
                ),
            });
        }

        // Query modified accounts from account_audit table.
        let aas = Self::query_aas_by_slot(&mut client, query_slot)?;

        // All data is queried, now start to use them.
        let mut update_commitment = Hash::default();
        let mut pre_accounts = HashMap::new();
        let mut post_accounts = HashMap::new();
        let mut pre_root = String::new();
        let mut pre_account_proof = String::new();
        let mut tx = Transaction::default();

        for (i, t) in txs.iter().enumerate() {
            // Parse account keys from t
            let pks = &t.message().account_keys;

            // Found all related (modified) accounts from account_audit
            let modified_accounts: Vec<AccountAuditRow> = aas
                .iter()
                .filter(|a| pks.contains(&Pubkey::try_from(a.pubkey.as_slice()).unwrap()) && a.txn_signature.is_some())
                .cloned()
                .collect();

            // Every time we iterate a tx, we calculate the commitment = Hash(tx, modified_accounts, pre_commitment).
            update_commitment = compute_ha(
                &t.signatures[0],
                &modified_accounts
                    .iter()
                    .map(|a| a.to_smt_account())
                    .collect::<Vec<SMTAccount>>(),
                &update_commitment,
            );

            // Found index-th transaction, let's do querying.
            if i == tx_no {
                // Get pre-accounts state from SMT.
                let mut smt_keys = vec![];
                let mut leaves = vec![];
                for key in pks {
                    // Get pre-state of these accounts from SMT.
                    let smt_key = SMTAccount {
                        pubkey: key.clone(),
                        ..Default::default()
                    }
                        .smt_key();
                    smt_keys.push(smt_key);
                    let account = smt.read().unwrap().get(&smt_key).unwrap();
                    leaves.push((smt_key.clone(), account.to_h256()));
                    pre_accounts.insert(
                        key.to_string(),
                        (
                            Hash::new(account.to_h256().as_slice().clone()).to_string(),
                            SerAccount::from_normal_account(account.to_normal_account()),
                        ),
                    );
                }

                // Get post-state of these accounts from results.
                modified_accounts.iter().for_each(|ma| {
                    // Check by same pubkey, remember, we select for transaction related post-accounts,
                    // So the account_audit row must have txn_signature.
                    if ma.txn_signature.is_some() {
                        post_accounts.insert(
                            Pubkey::try_from(ma.pubkey.as_slice()).unwrap().to_string(),
                            (
                                Hash::new(ma.to_smt_account().to_h256().as_slice().clone())
                                    .to_string(),
                                SerAccount::from_normal_account(
                                    ma.to_smt_account().to_normal_account(),
                                ),
                            ),
                        );
                    }
                });

                // Get pre-root of SMT.
                pre_root = Hash::new(smt.read().unwrap().root().as_slice().clone()).to_string(); //44 bytes

                // Get pre-account proof from SMT.
                let ck_proof: Vec<u8> = smt
                    .read()
                    .unwrap()
                    .merkle_proof(smt_keys.clone())
                    .unwrap()
                    .compile(smt_keys.clone())
                    .unwrap()
                    .into();
                pre_account_proof = hex::encode(&ck_proof);

                // Assign the transaction
                tx = t.clone();

                break;
            }

            // Not match index this round, just update SMT.
            for ma in modified_accounts {
                let raw_acct = ma.to_smt_account();
                let key_hash = raw_acct.smt_key();
                if let Err(err) = smt.write().unwrap().update(key_hash, raw_acct) {
                    return Err(SMTerError::UpdatingSMTError {
                        msg: format!(
                            "Failed to update SMT at slot {:?}, error: {:?}",
                            query_slot, err
                        ),
                    });
                }
            }
        }

        let proof0: String = pre_account_proof.clone();
        let root0: String = hex::encode(Hash::from_str(&pre_root).unwrap().clone());
        let mut leaves0 = vec![];
        for (k, v) in pre_accounts.iter() {
            leaves0.push((
                hex::encode(Pubkey::from_str(&k.clone()).unwrap().clone()),
                hex::encode(Hash::from_str(&v.0.clone()).unwrap().clone())
            )
            );
        }
        println!("proof0: {:?}", proof0);
        println!("root0: {:?}", root0);
        println!("leaves0: {:?}", leaves0);
        let result0: bool = MerkleVerify::merkle_verify(proof0, root0, leaves0);
        println!("result0: {:?}", result0);

        Ok(TxContext {
            pre_accounts,
            post_accounts,
            tx,
            pre_root,
            pre_account_proof,
            update_commitment: update_commitment.to_string(),
            block_height: block_height as u64,
        })
    }

    fn construct_tx(msg: DbTransactionMessage, signatures: Vec<Vec<u8>>) -> Transaction {
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

    fn query_aas_by_slot(
        client: &mut Client,
        query_slot: u64,
    ) -> Result<Vec<AccountAuditRow>, SMTerError> {
        // Query modified accounts from account_audit table.
        let aa_stmt = client.prepare(AA_STMT).unwrap();
        let results = client.query(&aa_stmt, &[&(query_slot as i64)]).unwrap();
        if results.is_empty() {
            return Err(SMTerError::QuerySlotNotExistsError {
                msg: format!("Couldn't find query slot {}'s account_audit", query_slot),
            });
        }

        let aas = results
            .iter()
            .map(|r| {
                let pubkey: Vec<u8> = r.get(0);
                let owner: Vec<u8> = r.get(1);
                let lamports: i64 = r.get(2);
                let slot: i64 = r.get(3);
                let executable: bool = r.get(4);
                let rent_epoch: i64 = r.get(5);
                let data: Vec<u8> = r.get(6);
                let write_version: i64 = r.get(7);
                let txn_signature: Option<Vec<u8>> = r.get(8);
                AccountAuditRow {
                    pubkey,
                    owner,
                    lamports,
                    slot,
                    executable,
                    rent_epoch,
                    data,
                    write_version,
                    txn_signature,
                    ..AccountAuditRow::default()
                }
            })
            .collect::<Vec<AccountAuditRow>>();
        Ok(aas)
    }

    fn query_txs_by_slot(
        client: &mut Client,
        query_slot: u64,
    ) -> Result<Vec<Transaction>, SMTerError> {
        let tx_stmt = client.prepare(TX_STMT).unwrap();
        let results = client.query(&tx_stmt, &[&(query_slot as i64)]).unwrap();

        let txs: Vec<Transaction> = results
            .iter()
            .map(|r| {
                // TODO: This hardly panic, but better consider use error-handling.
                let msg: DbTransactionMessage = r.get(0);
                let signatures: Vec<Vec<u8>> = r.get(1);
                Self::construct_tx(msg, signatures)
            })
            .collect::<Vec<Transaction>>();
        Ok(txs)
    }

    fn query_blockheight_by_slot(client: &mut Client, query_slot: u64) -> Result<i64, SMTerError> {
        let block_stmt = client.prepare(BLOCK_STMT).unwrap();
        let results = client.query(&block_stmt, &[&(query_slot as i64)]).unwrap();
        if results.is_empty() {
            return Err(SMTerError::QuerySlotNotExistsError {
                msg: format!("Can not found block with slot {}", query_slot),
            });
        }
        let block_height: i64 = results[0].get(0);
        Ok(block_height)
    }
}

fn compute_ha(tx_id: &Signature, accounts: &Vec<SMTAccount>, old_ha: &Hash) -> Hash {
    println!("compute_ha: {:?}", accounts);
    let mut buf = [0u8; 32];
    let mut blake2b = Blake2bBuilder::new(32).build();
    blake2b.update(&tx_id.as_ref());
    let mut accounts = accounts.clone();
    accounts.sort_by(|a, b| a.pubkey.cmp(&b.pubkey));
    accounts.iter().for_each(|a| {
        println!("sorted account: {}", a.pubkey);
        blake2b.update(&a.to_vec());
    });
    blake2b.update(old_ha.as_ref());
    blake2b.finalize(&mut buf);
    Hash::new(&buf)
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
