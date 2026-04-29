#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, Bytes, BytesN, Env, Symbol, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MerkleError {
    VerifyFailed = 1,
    RootNotSet = 2,
    EmptyProof = 3,
    ProofIndicesDifferentLength = 4,
}

#[contract]
pub struct MerkleProof;

#[contractimpl]
impl MerkleProof {
    pub fn root(env: Env) -> Symbol {
        Symbol::new(&env, "root")
    }

    pub fn __constructor(env: Env, root: BytesN<32>) -> Result<(), MerkleError> {
        env.storage().persistent().set(&Self::root(env), &root);
        Ok(())
    }

    pub fn verify(
        env: Env,
        leaf: BytesN<32>,
        proof: Vec<BytesN<32>>,
        indices: Vec<bool>,
    ) -> Result<(), MerkleError> {
        if proof.is_empty() {
            return Err(MerkleError::EmptyProof);
        }
        if proof.len() != indices.len() {
            return Err(MerkleError::ProofIndicesDifferentLength);
        }

        let prefix = BytesN::from_array(&env, &[0x00; 1]);
        let mut prefix_bytes: Bytes = prefix.to_bytes();
        let leaf_bytes: Bytes = leaf.to_bytes();
        prefix_bytes.append(&leaf_bytes);

        let leaf_hash = env.crypto().sha256(&prefix_bytes);
        let leaf_bytes_n: BytesN<32> = leaf_hash.to_bytes();
        let mut current = leaf_bytes_n;
        for (hash, is_left) in proof.iter().zip(indices.iter()) {
            match is_left {
                true => {
                    let sibling = hash;
                    let mut bytes_sibling: Bytes = sibling.to_bytes();
                    let bytes_current: Bytes = current.to_bytes();
                    bytes_sibling.append(&bytes_current);

                    let prefix = BytesN::from_array(&env, &[0x01; 1]);
                    let mut prefix_bytes: Bytes = prefix.to_bytes();
                    prefix_bytes.append(&bytes_sibling);
                    let parent = env.crypto().sha256(&prefix_bytes);
                    current = parent.into();
                }
                false => {
                    let sibling = hash;
                    let bytes_sibling: Bytes = sibling.to_bytes();
                    let mut bytes_current: Bytes = current.to_bytes();
                    bytes_current.append(&bytes_sibling);

                    let prefix = BytesN::from_array(&env, &[0x01; 1]);
                    let mut prefix_bytes: Bytes = prefix.to_bytes();
                    prefix_bytes.append(&bytes_current);
                    let parent = env.crypto().sha256(&prefix_bytes);
                    current = parent.into();
                }
            }
        }
        let root: BytesN<32> = env
            .storage()
            .persistent()
            .get(&Self::root(env))
            .ok_or(MerkleError::RootNotSet)?;
        if current == root {
            Ok(())
        } else {
            Err(MerkleError::VerifyFailed)
        }
    }
}

mod test;
