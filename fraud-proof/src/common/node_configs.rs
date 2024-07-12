use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NodeConfiguration {
    pub settle_chain: SettleChainConfiguration,
    pub store: StoreConfiguration,
    pub settle_preparation: SettlePreparationConfiguration,
    pub execute_chain: ExecuteChainConfiguration,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecuteChainConfiguration {
    pub genesis_hash: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SettlePreparationConfiguration {
    // keypair base58 string
    pub execute_node_keypair: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SettleChainConfiguration {
    pub url: String,
    pub fraud_proof_native_program_id: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StoreConfiguration {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub schema: String,
}


impl NodeConfiguration {
    pub fn load_from_file(file_name: &str) -> Result<NodeConfiguration, ConfigError> {
        let config = Config::builder()
            .add_source(config::File::with_name(file_name))
            .build()?;

        config.try_deserialize::<NodeConfiguration>()
    }
}
