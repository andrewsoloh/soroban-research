#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, Bytes, Env, Vec};

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
    }
}
mod test;
