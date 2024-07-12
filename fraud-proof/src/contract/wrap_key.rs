use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Debug, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
pub struct WrapKey {
    pub slot: u64,
    pub index: u32,
}

impl WrapKey {
    // slot: u64 needs 8 bytes
    // index: u32 needs 4 bytes
    pub fn size() -> usize {
        let slot_size: usize = 8;
        let index_size: usize = 4;

        let total_size: usize = slot_size + index_size;
        return total_size;
    }

    //init
    pub fn init_size() -> usize {
        return Self::size();
    }

    //max
    pub fn total_size() -> usize {
        return Self::size();
    }
}


#[cfg(test)]
pub mod test {
    use crate::fraud_proof::wrap_key::WrapKey;

    fn prepare_wrap_key() -> WrapKey {
        let slot: u64 = 20;
        let index: u32 = 2;

        let wrap_key = WrapKey {
            slot,
            index,
        };

        return wrap_key;
    }

    #[test]
    fn test_basic() {
        let wrap_key = prepare_wrap_key();
        let data = bincode::serialize(&wrap_key).unwrap();
        assert_eq!(WrapKey::size(), data.len());
    }

    #[test]
    fn test_convert() {
        let wrap_key_actual = prepare_wrap_key();
        let data = bincode::serialize(&wrap_key_actual).unwrap();
        let wrap_key_expect = bincode::deserialize::<WrapKey>(&data[..]).unwrap();
        assert_eq!(wrap_key_expect, wrap_key_actual);
    }
}