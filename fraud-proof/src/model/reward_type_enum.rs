use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
pub enum RewardTypeEnum {
    Fee,
    Rent,
    Staking,
    Voting,
}

