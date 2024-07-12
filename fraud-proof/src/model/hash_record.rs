/*
* slot_i                                slot_i+1
* |                root_i-1             |               root_i
* ↓                ↓             ↓ha_i  ↓               ↓          ↓ha_i+1
* ┌----------------┬─────────────┬-----┬┬---------------┬──────────┬------┐
* │    system tx   │   user tx   │  st │|   system tx   │ user tx  │  st  │
* └----------------┴─────────────┴-----┴┴---------------┴──────────┴------┘
* ↑                ↑             ↑      ↑               ↑          ↑      ↑
* first_hash  second_hash  third_hash fourth_hash
*
* first_hash_i = fourth_hash_i-1
* root_i = second_hash_i+1
* ha_i = third_hash_i, third_hash is calculated within this slot, has nothing to do with SMT.
* system tx/st: are transactions that used for system, which can not be challenged.
* user tx: can be challenged
*/
use serde_derive::{Deserialize, Serialize};

//The third_hash is the hash_account of the current slot
//The root_hash of the current slot is the second_hash of the next slot
//The first_hash of the current slot is the fourth_hash of the previous slot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HashRecord {
    pub first_hash: String,
    pub second_hash: String,
    pub third_hash: String,
    pub fourth_hash: String,
}
