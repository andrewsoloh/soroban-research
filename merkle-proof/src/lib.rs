#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, Bytes, Env, Symbol, Vec};

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
    pub fn __constructor(env: Env, root: Bytes) -> Result<(), MerkleError> {
        env.storage().persistent().set(&Self::root(env), &root);
        Ok(())
    }
    pub fn verify(
        env: Env,
        leaf: Bytes,
        proof: Vec<Bytes>,
        indices: Vec<bool>,
    ) -> Result<(), MerkleError> {
        let mut current = leaf;
        for (hash, is_left) in proof.iter().zip(indices.iter()) {
            match is_left {
                true => {
                    let mut sibling = hash;
                    sibling.append(&mut current);
                    let parent = env.crypto().sha256(&sibling);
                    current = parent.to_bytes().into_bytes();
                }
                false => {
                    let mut sibling = hash;
                    current.append(&mut sibling);
                    let parent = env.crypto().sha256(&current);
                    current = parent.to_bytes().into_bytes();
                }
            }
        }
        let root = env
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
