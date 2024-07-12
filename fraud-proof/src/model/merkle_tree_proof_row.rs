use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
#[diesel(table_name = table_merkle_tree_proof)]
pub struct MerkleTreeProofRow {
    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_id)]
    pub id: i64,

    #[diesel(sql_type = Int8)]
    #[diesel(column_name = column_slot)]
    pub slot: i64,

    #[diesel(sql_type = Nullable < Varchar >)]
    #[diesel(column_name = column_root_hash)]
    pub root_hash: Option<String>,

    #[diesel(sql_type = Timestamp)]
    #[diesel(column_name = column_updated_on)]
    pub updated_on: chrono::NaiveDateTime,
}

