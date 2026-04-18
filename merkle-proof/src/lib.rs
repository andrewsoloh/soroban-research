#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};

#[contract]
pub struct MerkleProof;


#[contractimpl]
impl MerkleProof {}

mod test;
