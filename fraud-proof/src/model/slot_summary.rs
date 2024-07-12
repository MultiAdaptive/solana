use serde_derive::{Deserialize, Serialize};

use crate::model::hash_record::HashRecord;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SlotSummary {
    pub slot: u64,
    pub record: HashRecord,
    pub count: usize,
}

