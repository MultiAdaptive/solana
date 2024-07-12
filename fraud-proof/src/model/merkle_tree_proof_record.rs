use diesel::Insertable;
use serde::{Deserialize, Serialize};

use crate::entities::merkle_tree_proof_entity::table_merkle_tree_proof;

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = table_merkle_tree_proof)]
pub struct MerkleTreeProofRecord {
    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_slot)]
    pub column_slot: i64,

    #[diesel(sql_type = Nullable < Varchar >)]
    #[diesel(column_name = column_root_hash)]
    pub column_root_hash: Option<String>,

    #[diesel(sql_type = Timestamp)]
    #[diesel(column_name = column_updated_on)]
    pub column_updated_on: chrono::NaiveDateTime,
}

