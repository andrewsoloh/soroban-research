#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, Bytes, Env, EnvBase, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MerkleError {
    VerifyFailed = 1,
}
#[contract]
pub struct MerkleProof;

#[contractimpl]
impl MerkleProof {
    pub fn verify(
        env: Env,
        leaf: Bytes,
        proof: Vec<Bytes>,
        indices: Vec<bool>,
    ) -> Result<bool, MerkleError> {
        for (hash, is_left) in proof.iter().zip(indices.iter()) {}
    }
}
mod test;
