use std::str::FromStr;
use blake2b_rs::Blake2bBuilder;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use sparse_merkle_tree::{CompiledMerkleProof, H256};
use sparse_merkle_tree::blake2b::Blake2bHasher;

pub struct MerkleVerify;

impl MerkleVerify {
    pub fn merkle_verify(proof: String, root: String, leaves: Vec<(String, String)>) -> bool {
        let proof = hex::decode(proof).unwrap();
        let root: [u8; 32] = hex::decode(root).unwrap().try_into().unwrap();
        let leaves = leaves.into_iter().map(|l| {
            let mut left: [u8; 32] = [0u8; 32];
            let mut blake2b = Blake2bBuilder::new(32).build();
            blake2b.update(hex::decode(l.0).unwrap().as_slice().clone());
            blake2b.finalize(&mut left);
            let right: [u8; 32] = hex::decode(l.1).unwrap().try_into().unwrap();
            (H256::from(left), H256::from(right))
        }).collect::<Vec<_>>();

        let proof = CompiledMerkleProof(proof);
        let root = H256::from(root);
        let result = proof.verify::<Blake2bHasher>(&root, leaves).unwrap();

        return result;
    }
}





