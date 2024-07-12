// use std::io::Read;
//
// use clap::builder::TypedValueParser;
// use log::{error, info, log};
//
// use fraud_proof_client::common::node_act::NodeAct;
// use fraud_proof_client::common::node_configs::NodeConfiguration;
// use fraud_proof_client::simulator::Simulator;
// use fraud_proof_client::utils::log_util::{init_logger, LogOutput};
//
// fn main() {
//     let config_file_path_arg = clap::Arg::new("config_file_path")
//         .long("config")
//         .value_name("CONFIG_FILE_PATH")
//         .num_args(0..=1)
//         .required(true)
//         .default_value("application.yaml")
//         .help("Configuration file to use");
//
//
//     let config_file = matches.get_one::<String>("config_file_path");
//     let default_config_file = String::from("application.yaml");
//     let config_file_path = config_file.unwrap_or(&default_config_file).as_str();
//     info!("config_file_path: {:?}", config_file_path.clone());
//     let cfg_result = &NodeConfiguration::load_from_file(config_file_path.clone());
//     info!("cfg_result: {:?}", cfg_result.to_owned());
//
//
//     match cfg_result {
//         Err(err) => {
//             error!("Load config error {:#?}", &err);
//         }
//         Ok(cfg) => {
//             let store = cfg.store.clone();
//             let settle_chain = cfg.settle_chain.clone();
//             let execute_chain = cfg.execute_chain.clone();
//             let settle_preparation = cfg.settle_preparation.clone();
//
//             match matches.subcommand() {
//                 Some(("execute-node", matches)) => {
//                     info!("execute-node");
//                     let slot_opt = matches.get_one::<String>("slot");
//                     let current_slot: u64 = slot_opt.unwrap_or(&String::from("0")).parse::<u64>().unwrap();
//                     info!("current_slot: {:?}", current_slot);
//                     if current_slot < 2 {
//                         panic!("The slot 0 and slot 1 are initial of blockchain, we never challenge. so slot must greater than 1. Now slot is :{:?}", current_slot)
//                     }
//                     let tx_no_opt = matches.get_one::<String>("tx_no");
//                     let current_tx_no: u32 = tx_no_opt.unwrap_or(&String::from("0")).parse::<u32>().unwrap();
//                     info!("current_tx_no: {:?}", current_tx_no);
//
//                     let act_opt = matches.get_one::<String>("act");
//                     let act = NodeAct::from(act_opt.cloned().unwrap());
//                     info!("act: {:?}", act);
//                     let mut simulator = Simulator::new()
//                         .store(&store)
//                         .chain(&settle_chain)
//                         .genesis(&execute_chain)
//                         .preparation(&settle_preparation)
//                         .act(&act)
//                         .slot(current_slot)
//                         .tx_no(current_tx_no);
//                     simulator.start_execute_node();
//                 }
//
//                 Some(("demonstrate-node", matches)) => {
//                     info!("demonstrate-node");
//                     let slot_opt = matches.get_one::<String>("slot");
//                     let current_slot: u64 = slot_opt.unwrap_or(&String::from("0")).parse::<u64>().unwrap();
//                     info!("current_slot: {:?}", current_slot);
//                     if current_slot < 2 {
//                         panic!("The slot 0 and slot 1 are initial of blockchain, we never challenge. so slot must greater than 1. Now slot is :{:?}", current_slot)
//                     }
//
//                     let tx_no_opt = matches.get_one::<String>("tx_no");
//                     let current_tx_no: u32 = tx_no_opt.unwrap_or(&String::from("0")).parse::<u32>().unwrap();
//                     info!("current_tx_no: {:?}", current_tx_no);
//
//                     let act_opt = matches.get_one::<String>("act");
//                     let act = NodeAct::from(act_opt.cloned().unwrap());
//                     info!("act: {:?}", act);
//
//                     let mut simulator = Simulator::new()
//                         .store(&store)
//                         .genesis(&execute_chain)
//                         .act(&act)
//                         .slot(current_slot)
//                         .tx_no(current_tx_no);
//                     simulator.start_demonstrate_node();
//                 }
//
//                 _ => unreachable!("don't get here"),
//             }
//         }
//     }
// }
//
//
