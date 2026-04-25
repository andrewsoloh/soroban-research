#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, Bytes, BytesN, Env, Symbol, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MerkleError {
    VerifyFailed = 1,
    RootNotSet = 2,
}
#[contract]
pub struct MerkleProof;

#[contractimpl]
impl MerkleProof {
    pub fn root(env: Env) -> Symbol {
        let root = Symbol::new(&env, "root");
        root
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
        let mut current = leaf;
        for (hash, is_left) in proof.iter().zip(indices.iter()) {
            match is_left {
                true => {
                    let sibling = hash;
                    let mut bytes_sibling = Bytes::from(&sibling);
                    let bytes_current = Bytes::from(&current);
                    bytes_sibling.append(&bytes_current);

                    let parent = env.crypto().sha256(&bytes_sibling);
                    current = parent.into();
                }
                false => {
                    let sibling = hash;
                    let bytes_sibling = Bytes::from(&sibling);
                    let mut bytes_current = Bytes::from(&current);
                    bytes_current.append(&bytes_sibling);

                    let parent = env.crypto().sha256(&bytes_current);
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
            return Ok(());
        } else {
            Err(MerkleError::VerifyFailed)
        }
    }
}
mod test;
