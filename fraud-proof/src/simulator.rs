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
// use crate::contract::chain_brief::ChainBrief;
// use crate::contract::chain_tally::ChainTally;
// use crate::contract::wrap_key::WrapKey;
// use crate::contract::wrap_slot::WrapSlot;
// use crate::services::chain_basic_service::ChainBasicService;
// use crate::services::chain_brief_service::ChainBriefService;
// use crate::services::chain_state_service::ChainStateService;
// use crate::services::chain_tally_service::ChainTallyService;
// use crate::services::demonstrate_service::DemonstrateService;
// use crate::services::store_service::StoreService;
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
        if cfg_result.is_ok(){
            let cfg = cfg_result.unwrap();
            self.store_config = Some(cfg.store.clone());
            self.chain_config = Some(cfg.chain.clone());
        }

        self
    }


    // pub fn start_execute_node(&mut self) {
    //     if let Err(e) = self.connect_store() {
    //         error!("{:?}", e);
    //     };
    //     if let Err(e) = self.connect_chain() {
    //         error!("{:?}", e);
    //     };
    //
    //     let node_act = self.node_act.unwrap();
    //
    //     match node_act {
    //         NodeAct::CreateBrief => {
    //             let is_success = self.start_execute_node_create_brief();
    //             if is_success {
    //                 info!("execute node create brief success. slot: {:?}", self.current_slot.unwrap());
    //             } else {
    //                 error!("execute node create brief fail. slot: {:?}", self.current_slot.unwrap());
    //             }
    //         }
    //         NodeAct::SubmitBrief => {
    //             let is_success = self.start_execute_node_submit_brief();
    //             if is_success {
    //                 info!("execute node submit brief success.");
    //             } else {
    //                 error!("execute node submit brief fail.");
    //             }
    //         }
    //         _ => unreachable!("don't get here"),
    //     }
    // }
    //
    // pub fn start_demonstrate_node(&mut self) {
    //     if let Err(e) = self.connect_store() {
    //         error!("{:?}", e);
    //     };
    //
    //     let node_act = self.node_act.unwrap();
    //
    //     match node_act {
    //         NodeAct::GenerateData => {
    //             let is_success = self.start_demonstrate_node_generate_data();
    //             if is_success {
    //                 info!("demonstrate node generate data success. slot: {:?}", self.current_slot.unwrap());
    //             } else {
    //                 error!("demonstrate node generate data fail. slot: {:?}", self.current_slot.unwrap());
    //             }
    //         }
    //         _ => unreachable!("don't get here"),
    //     }
    // }
    //
    //
    // fn connect_chain(&mut self) -> Result<(), NodeError> {
    //     let config = self.settle_chain_config.clone().unwrap();
    //     let rpc_url = config.url;
    //     let rpc_client: RpcClient = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    //
    //     self.chain_client = Some(rpc_client);
    //
    //     Ok(())
    // }
    //
    // fn connect_store(&mut self) -> Result<(), NodeError> {
    //     let config = self.store_config.clone().unwrap();
    //
    //     let pool: PgConnectionPool = create_pool(
    //         config.to_owned(),
    //         10,
    //     );
    //
    //     self.store_client_pool = Some(pool);
    //
    //     let one = create_one(config.to_owned());
    //
    //     self.store_client_one = Some(one);
    //
    //     Ok(())
    // }
    //
    // pub fn start_execute_node_create_brief(&mut self) -> bool {
    //     let mut is_success: bool = true;
    //
    //
    //     is_success = self.create_state_account();
    //     if !is_success {
    //         error!("create state account fail.");
    //         return is_success.clone();
    //     }
    //
    //     is_success = self.create_tally_account();
    //     if !is_success {
    //         error!("create tally account fail.");
    //         return is_success.clone();
    //     }
    //
    //
    //     let current_slot = self.current_slot.unwrap();
    //
    //     let mut store_service = StoreService {
    //         client_pool: self.store_client_pool.as_ref().unwrap(),
    //         client_one: self.store_client_one.as_mut().unwrap(),
    //     };
    //
    //     let current_wrap_slot: WrapSlot = WrapSlot {
    //         slot: current_slot,
    //     };
    //
    //     let briefs: Vec<ChainBrief> = store_service.generate_until_briefs(current_wrap_slot.clone()).unwrap();
    //
    //     //send brief to chain
    //     // The slot 0 and slot 1 are initial of blockchain, we never challenge, so skip them.
    //     for brief in briefs {
    //         let wrap_slot: WrapSlot = WrapSlot {
    //             slot: brief.slot,
    //         };
    //         is_success = self.create_brief_account(brief.clone());
    //         if !is_success {
    //             error!("create brief account fail. slot: {:?}", wrap_slot.clone());
    //             break;
    //         }
    //     }
    //
    //     return is_success.clone();
    // }
    //
    //
    // pub fn start_execute_node_submit_brief(&mut self) -> bool {
    //     let mut is_success: bool = true;
    //     let rpc_client = self.chain_client.as_ref().unwrap();
    //
    //     let fraud_proof_native_program_id_binding = Pubkey::from_str(&self.settle_chain_config.clone().unwrap().fraud_proof_native_program_id);
    //     let fraud_proof_native_program_id = fraud_proof_native_program_id_binding.as_ref().unwrap();
    //
    //     let execute_node = self.get_role(&self.settle_preparation_config.clone().unwrap().execute_node_keypair).unwrap();
    //
    //     let chain_state_service = ChainStateService {
    //         rpc_client: rpc_client,
    //         program_id: fraud_proof_native_program_id,
    //         payer: &execute_node,
    //     };
    //
    //     let chain_brief_service = ChainBriefService {
    //         rpc_client: rpc_client,
    //         program_id: fraud_proof_native_program_id,
    //         payer: &execute_node,
    //     };
    //
    //     let chain_tally_service = ChainTallyService {
    //         rpc_client: rpc_client,
    //         program_id: fraud_proof_native_program_id,
    //         payer: &execute_node,
    //     };
    //
    //     let is_state_exist = chain_state_service.is_state_account_exist();
    //     if !is_state_exist {
    //         info!("state account is not exist.");
    //         is_success = chain_state_service.initialize();
    //         if !is_success {
    //             error!("initialize fail.");
    //             return is_success.clone();
    //         } else {
    //             info!("initialize success.");
    //         }
    //     } else {
    //         info!("state account is already exist.");
    //     }
    //
    //     let is_tally_exist = chain_tally_service.is_tally_account_exist();
    //     if !is_tally_exist {
    //         info!("tally account is not exist.");
    //         is_success = chain_tally_service.create_tally_account();
    //         if !is_success {
    //             error!("create tally account fail.");
    //             return is_success.clone();
    //         } else {
    //             info!("create tally account success.");
    //         }
    //     } else {
    //         info!("tally account is already exist.");
    //     }
    //
    //     let current_wrap_slot = chain_tally_service.get_max_wrap_slot().unwrap();
    //     let current_slot = current_wrap_slot.slot;
    //
    //     let mut store_service = StoreService {
    //         client_pool: self.store_client_pool.as_ref().unwrap(),
    //         client_one: self.store_client_one.as_mut().unwrap(),
    //     };
    //
    //     let present_slot = store_service.query_max_slot_from_block().unwrap();
    //
    //     if current_slot >= present_slot {
    //         info!("all slots are submitted. current slot: {:?} present slot: {:?}", current_slot.clone(),present_slot.clone());
    //         return is_success.clone();
    //     }
    //     info!("some slots are not submitted. current slot: {:?} present slot: {:?}", current_slot.clone(),present_slot.clone());
    //
    //     let ret = store_service.generate_initial_slot_summary_and_smt(current_slot);
    //     if ret.is_err() {
    //         error!("generate initial slot summary and smt fail. slot: {:?}", current_slot.clone());
    //         is_success = false;
    //         return is_success.clone();
    //     }
    //     let (mut smt_tree, mut slot_summary);
    //
    //     (smt_tree, slot_summary) = ret.unwrap();
    //
    //     let mut start_slot = current_slot.clone() + 1;
    //     let mut end_slot = (present_slot.clone() - 1).min(start_slot.clone() + 100);
    //
    //     loop {
    //         (smt_tree, slot_summary) = store_service.generate_continue_slot_summary_and_smt(start_slot.clone(), end_slot.clone(), smt_tree, slot_summary.clone()).unwrap();
    //
    //         let briefs: Vec<ChainBrief> = store_service.generate_range_briefs_from_slot_summary(start_slot.clone(), end_slot.clone(), slot_summary.clone()).unwrap();
    //
    //         // send brief to chain
    //         // The slot 0 and slot 1 are initial of blockchain, we never challenge, so skip them.
    //         for brief in briefs {
    //             let wrap_slot: WrapSlot = WrapSlot {
    //                 slot: brief.slot,
    //             };
    //             let is_brief_exist = chain_brief_service.is_brief_account_exist(wrap_slot.clone());
    //             if is_brief_exist {
    //                 info!("brief account is already exist. slot: {:?}", wrap_slot.clone());
    //                 continue;
    //             }
    //             info!("brief account is not exist. slot: {:?},brief: {:?}", wrap_slot.clone(), brief);
    //             is_success = chain_brief_service.create_brief_account(wrap_slot.clone(), brief.clone());
    //             if is_success {
    //                 let is_brief_exist = chain_brief_service.is_brief_account_exist(wrap_slot.clone());
    //                 if is_brief_exist {
    //                     let save_brief = chain_brief_service.fetch_brief_account(wrap_slot.clone()).unwrap();
    //                     info!("create brief account success. slot: {:?}, brief: {:?}", wrap_slot.clone(), save_brief.clone());
    //                 } else {
    //                     error!("create brief account fail. slot: {:?}", wrap_slot.clone());
    //                     is_success = false;
    //                     break;
    //                 }
    //             } else {
    //                 error!("create brief account fail. slot: {:?}", wrap_slot.clone());
    //                 break;
    //             }
    //         }
    //
    //         start_slot = end_slot + 1;
    //         loop {
    //             let max_slot = store_service.query_max_slot_from_block().unwrap();
    //             if max_slot > start_slot + 1 {
    //                 end_slot = (max_slot.clone() - 1).min(start_slot.clone() + 100);
    //                 break;
    //             }
    //             time_util::sleep_seconds(1);
    //         }
    //     }
    //     // async_std::task::block_on(async {
    //     //     self.create_brief_async(current_slot.clone(), present_slot.clone() as u64).await;
    //     // });
    //
    //     // // 使用 async-std 运行时执行异步函数
    //     // async_std::task::block_on(async {
    //     //     let mut tasks = vec![];
    //     //
    //     //     for i in current_slot + 1..present_slot as u64 {
    //     //         // 创建并发任务
    //     //         let task = async {
    //     //             let now_wrap_slot: WrapSlot = WrapSlot {
    //     //                 slot: i.clone(),
    //     //             };
    //     //             info!("slot: {:?}", now_wrap_slot.clone());
    //     //         };
    //     //         tasks.push(task);
    //     //     }
    //     //
    //     //     // 等待所有任务完成
    //     //     futures::future::join_all(tasks).await;
    //     // });
    //
    //
    //     // crossbeam::thread::scope(|s| {
    //     //     for i in current_slot + 1..present_slot as u64 {
    //     //         s.spawn(move |_| {
    //     //             let now_wrap_slot: WrapSlot = WrapSlot {
    //     //                 slot: i.clone(),
    //     //             };
    //     //             info!("slot: {:?}", now_wrap_slot.clone());
    //     //
    //     //             // let briefs: Vec<StoreBrief> = store_service.generate_briefs(now_wrap_slot.clone()).unwrap();
    //     //
    //     //             //send brief to chain
    //     //             // The slot 0 and slot 1 are initial of blockchain, we never challenge, so skip them.
    //     //             // for brief in briefs {
    //     //             // let wrap_slot: WrapSlot = WrapSlot {
    //     //             //     slot: brief.slot,
    //     //             // };
    //     //             //     let is_brief_exist = chain_brief_service.is_brief_account_exist(wrap_slot.clone());
    //     //             //     if is_brief_exist {
    //     //             //         info!("brief account is already exist. slot: {:?}", wrap_slot.clone());
    //     //             //         continue;
    //     //             //     }
    //     //             //     info!("brief account is not exist. slot: {:?},brief: {:?}", wrap_slot.clone(), brief);
    //     //             //     is_success = chain_brief_service.create_brief_account(wrap_slot.clone(), brief.clone());
    //     //             //     if is_success {
    //     //             //         let is_brief_exist = chain_brief_service.is_brief_account_exist(wrap_slot.clone());
    //     //             //         if is_brief_exist {
    //     //             //             let save_brief = chain_brief_service.fetch_brief_account(wrap_slot.clone()).unwrap();
    //     //             //             info!("create brief account success. slot: {:?}, brief: {:?}", wrap_slot.clone(), save_brief.clone());
    //     //             //         } else {
    //     //             //             error!("create brief account fail. slot: {:?}", wrap_slot.clone());
    //     //             //             is_success = false;
    //     //             //             break;
    //     //             //         }
    //     //             //     } else {
    //     //             //         error!("create brief account fail. slot: {:?}", wrap_slot.clone());
    //     //             //         break;
    //     //             //     }
    //     //             // }
    //     //         });
    //     //     }
    //     // }).unwrap();
    //
    //     return is_success.clone();
    // }


    // async fn create_brief_async(&mut self, current_slot: u64, present_slot: u64) {
    //     let mut tasks = vec![];

    // for i in current_slot.clone() + 1..present_slot.clone() {
    //     let now_wrap_slot = WrapSlot { slot: i.clone() };
    //
    //     // 创建并发任务
    //     let task = async {
    //         info!("slot: {:?}", now_wrap_slot.clone());
    //
    //         // let mut store_service = StoreService {
    //         //     client_pool: Option::from(self.store_client_pool.as_ref().unwrap()),
    //         //     client_one: Option::from(self.store_client_one.as_mut().unwrap()),
    //         // };
    //         //
    //         // let briefs: Vec<StoreBrief> = store_service.generate_briefs(now_wrap_slot.clone()).unwrap();
    //
    //         //send brief to chain
    //         // The slot 0 and slot 1 are initial of blockchain, we never challenge, so skip them.
    //         // for brief in briefs {
    //         // let wrap_slot: WrapSlot = WrapSlot {
    //         //     slot: brief.slot,
    //         // };
    //         //     let is_brief_exist = chain_brief_service.is_brief_account_exist(wrap_slot.clone());
    //         //     if is_brief_exist {
    //         //         info!("brief account is already exist. slot: {:?}", wrap_slot.clone());
    //         //         continue;
    //         //     }
    //         //     info!("brief account is not exist. slot: {:?},brief: {:?}", wrap_slot.clone(), brief);
    //         //     is_success = chain_brief_service.create_brief_account(wrap_slot.clone(), brief.clone());
    //         //     if is_success {
    //         //         let is_brief_exist = chain_brief_service.is_brief_account_exist(wrap_slot.clone());
    //         //         if is_brief_exist {
    //         //             let save_brief = chain_brief_service.fetch_brief_account(wrap_slot.clone()).unwrap();
    //         //             info!("create brief account success. slot: {:?}, brief: {:?}", wrap_slot.clone(), save_brief.clone());
    //         //         } else {
    //         //             error!("create brief account fail. slot: {:?}", wrap_slot.clone());
    //         //             is_success = false;
    //         //             break;
    //         //         }
    //         //     } else {
    //         //         error!("create brief account fail. slot: {:?}", wrap_slot.clone());
    //         //         break;
    //         //     }
    //         // }
    //     };
    //     tasks.push(task);
    // }

    // 等待所有任务完成
    // futures::future::join_all(tasks).await;
    // }


    // //generate data for comparison
    // pub fn start_demonstrate_node_generate_data(&mut self) -> bool {
    //     let mut is_success: bool = true;
    //
    //     let current_slot = self.current_slot.unwrap();
    //
    //     let current_wrap_slot = WrapSlot {
    //         slot: current_slot
    //     };
    //
    //     let mut demonstrate_service = DemonstrateService {
    //         client_pool: self.store_client_pool.as_ref().unwrap(),
    //         client_one: self.store_client_one.as_mut().unwrap(),
    //     };
    //
    //     let brief_notes = demonstrate_service.generate_briefs(current_wrap_slot.clone()).unwrap();
    //
    //     for brief_note in brief_notes {
    //         let serialised_brief_note = serde_json::to_string(&brief_note).unwrap();
    //         println!("brief_note: {:?}", serialised_brief_note);
    //     }
    //
    //     return is_success.clone();
    // }
    //
    //
    // fn create_state_account(&mut self) -> bool {
    //     let mut is_success: bool = true;
    //     let rpc_client = self.chain_client.as_ref().unwrap();
    //
    //     let fraud_proof_native_program_id_binding = Pubkey::from_str(&self.settle_chain_config.clone().unwrap().fraud_proof_native_program_id);
    //     let fraud_proof_native_program_id = fraud_proof_native_program_id_binding.as_ref().unwrap();
    //
    //     let execute_node = self.get_role(&self.settle_preparation_config.clone().unwrap().execute_node_keypair).unwrap();
    //
    //     let chain_state_service = ChainStateService {
    //         rpc_client: rpc_client,
    //         program_id: fraud_proof_native_program_id,
    //         payer: &execute_node,
    //     };
    //
    //     let is_state_exist = chain_state_service.is_state_account_exist();
    //     if !is_state_exist {
    //         info!("state account is not exist.");
    //         is_success = chain_state_service.initialize();
    //         if !is_success {
    //             error!("initialize fail.");
    //             return is_success.clone();
    //         } else {
    //             info!("initialize success.");
    //         }
    //     } else {
    //         info!("state account is already exist.");
    //     }
    //
    //     return is_success.clone();
    // }
    //
    // fn create_tally_account(&mut self) -> bool {
    //     let mut is_success: bool = true;
    //     let rpc_client = self.chain_client.as_ref().unwrap();
    //
    //     let fraud_proof_native_program_id_binding = Pubkey::from_str(&self.settle_chain_config.clone().unwrap().fraud_proof_native_program_id);
    //     let fraud_proof_native_program_id = fraud_proof_native_program_id_binding.as_ref().unwrap();
    //
    //     let execute_node = self.get_role(&self.settle_preparation_config.clone().unwrap().execute_node_keypair).unwrap();
    //
    //     let chain_tally_service = ChainTallyService {
    //         rpc_client: rpc_client,
    //         program_id: fraud_proof_native_program_id,
    //         payer: &execute_node,
    //     };
    //
    //     let is_tally_exist = chain_tally_service.is_tally_account_exist();
    //     if !is_tally_exist {
    //         info!("tally account is not exist.");
    //         is_success = chain_tally_service.create_tally_account();
    //         if !is_success {
    //             error!("create tally account fail.");
    //             return is_success.clone();
    //         } else {
    //             info!("create tally account success.");
    //         }
    //     } else {
    //         info!("tally account is already exist.");
    //     }
    //
    //     return is_success.clone();
    // }
    //
    //
    // fn create_brief_account(&mut self, brief: ChainBrief) -> bool {
    //     let mut is_success: bool = true;
    //     let rpc_client = self.chain_client.as_ref().unwrap();
    //
    //     let fraud_proof_native_program_id_binding = Pubkey::from_str(&self.settle_chain_config.clone().unwrap().fraud_proof_native_program_id);
    //     let fraud_proof_native_program_id = fraud_proof_native_program_id_binding.as_ref().unwrap();
    //
    //     let execute_node = self.get_role(&self.settle_preparation_config.clone().unwrap().execute_node_keypair).unwrap();
    //
    //
    //     let chain_brief_service = ChainBriefService {
    //         rpc_client: rpc_client,
    //         program_id: fraud_proof_native_program_id,
    //         payer: &execute_node,
    //     };
    //
    //     let wrap_slot: WrapSlot = WrapSlot {
    //         slot: brief.slot,
    //     };
    //     let is_brief_exist = chain_brief_service.is_brief_account_exist(wrap_slot.clone());
    //     if is_brief_exist {
    //         info!("brief account is already exist. slot: {:?}", wrap_slot.clone());
    //         return is_success.clone();
    //     }
    //     info!("brief account is not exist. slot: {:?},brief: {:?}", wrap_slot.clone(), brief);
    //     is_success = chain_brief_service.create_brief_account(wrap_slot.clone(), brief.clone());
    //     if is_success {
    //         let is_brief_exist = chain_brief_service.is_brief_account_exist(wrap_slot.clone());
    //         if is_brief_exist {
    //             let save_brief = chain_brief_service.fetch_brief_account(wrap_slot.clone()).unwrap();
    //             info!("create brief account success. slot: {:?}, brief: {:?}", wrap_slot.clone(), save_brief.clone());
    //         } else {
    //             error!("create brief account fail. slot: {:?}", wrap_slot.clone());
    //             is_success = false;
    //             return is_success.clone();
    //         }
    //     } else {
    //         error!("create brief account fail. slot: {:?}", wrap_slot.clone());
    //         return is_success.clone();
    //     }
    //
    //     return is_success.clone();
    // }
    //
    // pub fn get_role(&self, keypair: &str) -> Option<Keypair> {
    //     let mut is_success: bool = true;
    //     let rpc_client = self.chain_client.as_ref().unwrap();
    //
    //     let role = Keypair::from_base58_string(keypair);
    //     let chain_basic_service = ChainBasicService {
    //         rpc_client: rpc_client,
    //     };
    //
    //     is_success = chain_basic_service.request_airdrop(&role.pubkey().clone(), 10_000_000_000);
    //     if is_success {
    //         info!("request airdrop success. role pubkey: {:?}; role keypair: {:?}",
    //             role.pubkey().to_string(), role.to_base58_string());
    //         return Some(role);
    //     } else {
    //         error!("request airdrop fail. role pubkey: {:?}; role keypair: {:?}",
    //             role.pubkey().to_string(), role.to_base58_string());
    //         return None;
    //     }
    // }
}

