use std::str::FromStr;
use std::time::Duration;

use log::{error, info};
use postgres::Client;
use rand::Rng;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};

use crate::common::node_configs::{ChainConfiguration, StoreConfiguration};
use crate::common::node_error::NodeError;
use crate::common::node_configs::NodeConfiguration;
use crate::contract::chain_brief::ChainBrief;
use crate::contract::chain_tally::ChainTally;
use crate::contract::wrap_slot::WrapSlot;
use crate::services::chain_basic_service::ChainBasicService;
use crate::services::chain_brief_service::ChainBriefService;
use crate::services::chain_state_service::ChainStateService;
use crate::services::chain_tally_service::ChainTallyService;
use crate::services::store_service::StoreService;
use crate::utils::store_util::{create_one, create_pool, PgConnectionPool};
use crate::utils::time_util;

pub struct Simulator {
    store_client_pool: Option<PgConnectionPool>,
    store_client_one: Option<Client>,
    chain_client: Option<RpcClient>,
    store_config: Option<StoreConfiguration>,
    chain_config: Option<ChainConfiguration>,
}


impl Simulator {
    pub fn new() -> Self {
        Self {
            store_client_pool: None,
            store_client_one: None,
            chain_client: None,
            store_config: None,
            chain_config: None,
        }
    }

    pub fn config(mut self, config_file: String) -> Self {
        let cfg_result = NodeConfiguration::load_from_file(config_file.clone().as_str());
        info!("cfg_result: {:?}", cfg_result);
        if cfg_result.is_ok() {
            let cfg = cfg_result.unwrap();
            self.store_config = Some(cfg.store.clone());
            self.chain_config = Some(cfg.chain.clone());
        }

        self
    }


    pub fn start(&mut self) {
        if let Err(e) = self.connect_store() {
            error!("{:?}", e);
        };
        if let Err(e) = self.connect_chain() {
            error!("{:?}", e);
        };

        let is_success = self.start_submit_brief();
        if is_success {
            info!("submit brief success.");
        } else {
            error!("submit brief fail.");
        }
    }


    fn connect_chain(&mut self) -> Result<(), NodeError> {
        let config = self.chain_config.clone().unwrap();
        let rpc_url = config.url;
        let rpc_client: RpcClient = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

        self.chain_client = Some(rpc_client);

        Ok(())
    }

    fn connect_store(&mut self) -> Result<(), NodeError> {
        let config = self.store_config.clone().unwrap();

        let pool: PgConnectionPool = create_pool(
            config.to_owned(),
            10,
        );

        self.store_client_pool = Some(pool);

        let one = create_one(config.to_owned());

        self.store_client_one = Some(one);

        Ok(())
    }


    pub fn start_submit_brief(&mut self) -> bool {
        let mut is_success: bool = true;
        let rpc_client = self.chain_client.as_ref().unwrap();

        let fraud_proof_native_program_id_binding = Pubkey::from_str(&self.chain_config.clone().unwrap().fraud_proof_native_program_id);
        let fraud_proof_native_program_id = fraud_proof_native_program_id_binding.as_ref().unwrap();

        let execute_node = self.get_role(&self.chain_config.clone().unwrap().execute_keypair).unwrap();

        let chain_brief_service = ChainBriefService {
            rpc_client: rpc_client,
            program_id: fraud_proof_native_program_id,
            payer: &execute_node,
        };

        let chain_tally_service = ChainTallyService {
            rpc_client: rpc_client,
            program_id: fraud_proof_native_program_id,
            payer: &execute_node,
        };

        is_success = self.create_state_account();
        if !is_success {
            error!("create state account fail.");
            return is_success.clone();
        } else {
            info!("create state account success.");
        }

        let is_tally_exist = chain_tally_service.is_tally_account_exist();
        if !is_tally_exist {
            info!("tally account is not exist.");
            is_success = chain_tally_service.create_tally_account();
            if !is_success {
                error!("create tally account fail.");
                return is_success.clone();
            } else {
                info!("create tally account success.");
            }
        } else {
            info!("tally account is already exist.");
        }

        let current_wrap_slot = chain_tally_service.get_max_wrap_slot().unwrap();
        let current_slot = current_wrap_slot.slot;

        let mut store_service = StoreService {
            client_pool: self.store_client_pool.as_ref().unwrap(),
            client_one: self.store_client_one.as_mut().unwrap(),
        };

        let present_slot = store_service.query_max_slot_from_block().unwrap();

        if current_slot >= present_slot {
            info!("all slots are submitted. current slot: {:?} present slot: {:?}", current_slot.clone(),present_slot.clone());
            return is_success.clone();
        }
        info!("some slots are not submitted. current slot: {:?} present slot: {:?}", current_slot.clone(),present_slot.clone());

        let ret = store_service.generate_initial_slot_summary_and_smt(current_slot);
        if ret.is_err() {
            error!("generate initial slot summary and smt fail. slot: {:?}", current_slot.clone());
            is_success = false;
            return is_success.clone();
        }
        let (mut smt_tree, mut slot_summary);

        (smt_tree, slot_summary) = ret.unwrap();

        let mut start_slot = current_slot.clone() + 1;
        let mut end_slot = (present_slot.clone() - 1).min(start_slot.clone() + 100);

        loop {
            (smt_tree, slot_summary) = store_service.generate_continue_slot_summary_and_smt(start_slot.clone(), end_slot.clone(), smt_tree, slot_summary.clone()).unwrap();

            let briefs: Vec<ChainBrief> = store_service.generate_range_briefs_from_slot_summary(start_slot.clone(), end_slot.clone(), slot_summary.clone()).unwrap();

            // send brief to chain
            // The slot 0 and slot 1 are initial of blockchain, we never challenge, so skip them.
            for brief in briefs {
                let wrap_slot: WrapSlot = WrapSlot {
                    slot: brief.slot,
                };
                let is_brief_exist = chain_brief_service.is_brief_account_exist(wrap_slot.clone());
                if is_brief_exist {
                    info!("brief account is already exist. slot: {:?}", wrap_slot.clone());
                    continue;
                }
                info!("brief account is not exist. slot: {:?},brief: {:?}", wrap_slot.clone(), brief);
                is_success = chain_brief_service.create_brief_account(wrap_slot.clone(), brief.clone());
                if is_success {
                    let is_brief_exist = chain_brief_service.is_brief_account_exist(wrap_slot.clone());
                    if is_brief_exist {
                        let save_brief = chain_brief_service.fetch_brief_account(wrap_slot.clone()).unwrap();
                        info!("create brief account success. slot: {:?}, brief: {:?}", wrap_slot.clone(), save_brief.clone());
                    } else {
                        error!("create brief account fail. slot: {:?}", wrap_slot.clone());
                        is_success = false;
                        break;
                    }
                } else {
                    error!("create brief account fail. slot: {:?}", wrap_slot.clone());
                    break;
                }
            }

            start_slot = end_slot + 1;
            loop {
                let max_slot = store_service.query_max_slot_from_block().unwrap();
                if max_slot > start_slot + 1 {
                    end_slot = (max_slot.clone() - 1).min(start_slot.clone() + 100);
                    break;
                }
                time_util::sleep_seconds(1);
            }
        }
    }


    fn create_state_account(&self) -> bool {
        let mut is_success: bool = true;
        let rpc_client = self.chain_client.as_ref().unwrap();

        let fraud_proof_native_program_id_binding = Pubkey::from_str(&self.chain_config.clone().unwrap().fraud_proof_native_program_id);
        let fraud_proof_native_program_id = fraud_proof_native_program_id_binding.as_ref().unwrap();

        let execute_node = self.get_role(&self.chain_config.clone().unwrap().execute_keypair).unwrap();

        let chain_state_service = ChainStateService {
            rpc_client: rpc_client,
            program_id: fraud_proof_native_program_id,
            payer: &execute_node,
        };

        let is_state_exist = chain_state_service.is_state_account_exist();
        if !is_state_exist {
            info!("state account is not exist.");
            is_success = chain_state_service.initialize();
            if !is_success {
                error!("initialize fail.");
                return is_success.clone();
            } else {
                info!("initialize success.");
            }
        } else {
            info!("state account is already exist.");
        }

        return is_success.clone();
    }

    fn create_tally_account(&mut self) -> bool {
        let mut is_success: bool = true;
        let rpc_client = self.chain_client.as_ref().unwrap();

        let fraud_proof_native_program_id_binding = Pubkey::from_str(&self.chain_config.clone().unwrap().fraud_proof_native_program_id);
        let fraud_proof_native_program_id = fraud_proof_native_program_id_binding.as_ref().unwrap();

        let execute_node = self.get_role(&self.chain_config.clone().unwrap().execute_keypair).unwrap();

        let chain_tally_service = ChainTallyService {
            rpc_client: rpc_client,
            program_id: fraud_proof_native_program_id,
            payer: &execute_node,
        };

        let is_tally_exist = chain_tally_service.is_tally_account_exist();
        if !is_tally_exist {
            info!("tally account is not exist.");
            is_success = chain_tally_service.create_tally_account();
            if !is_success {
                error!("create tally account fail.");
                return is_success.clone();
            } else {
                info!("create tally account success.");
            }
        } else {
            info!("tally account is already exist.");
        }

        return is_success.clone();
    }


    fn create_brief_account(&mut self, brief: ChainBrief) -> bool {
        let mut is_success: bool = true;
        let rpc_client = self.chain_client.as_ref().unwrap();

        let fraud_proof_native_program_id_binding = Pubkey::from_str(&self.chain_config.clone().unwrap().fraud_proof_native_program_id);
        let fraud_proof_native_program_id = fraud_proof_native_program_id_binding.as_ref().unwrap();

        let execute_node = self.get_role(&self.chain_config.clone().unwrap().execute_keypair).unwrap();


        let chain_brief_service = ChainBriefService {
            rpc_client: rpc_client,
            program_id: fraud_proof_native_program_id,
            payer: &execute_node,
        };

        let wrap_slot: WrapSlot = WrapSlot {
            slot: brief.slot,
        };
        let is_brief_exist = chain_brief_service.is_brief_account_exist(wrap_slot.clone());
        if is_brief_exist {
            info!("brief account is already exist. slot: {:?}", wrap_slot.clone());
            return is_success.clone();
        }
        info!("brief account is not exist. slot: {:?},brief: {:?}", wrap_slot.clone(), brief);
        is_success = chain_brief_service.create_brief_account(wrap_slot.clone(), brief.clone());
        if is_success {
            let is_brief_exist = chain_brief_service.is_brief_account_exist(wrap_slot.clone());
            if is_brief_exist {
                let save_brief = chain_brief_service.fetch_brief_account(wrap_slot.clone()).unwrap();
                info!("create brief account success. slot: {:?}, brief: {:?}", wrap_slot.clone(), save_brief.clone());
            } else {
                error!("create brief account fail. slot: {:?}", wrap_slot.clone());
                is_success = false;
                return is_success.clone();
            }
        } else {
            error!("create brief account fail. slot: {:?}", wrap_slot.clone());
            return is_success.clone();
        }

        return is_success.clone();
    }

    pub fn get_role(&self, keypair: &str) -> Option<Keypair> {
        let role = Keypair::from_base58_string(keypair);
        return Some(role);
    }
}



