#![cfg(test)]
use super::*;
use soroban_sdk::{vec, Bytes, BytesN, Env};

fn initialize_root(env: &Env) -> BytesN<32> {
    let leaf = BytesN::from_array(&env, &[2; 32]);
    let leaf_bytes: Bytes = leaf.to_bytes();
    let proof = BytesN::from_array(&env, &[16; 32]);
    let mut proof_bytes: Bytes = proof.to_bytes();

    let leaf_prefix = BytesN::from_array(&env, &[0x00; 1]);
    let mut leaf_prefix_bytes: Bytes = leaf_prefix.to_bytes();
    leaf_prefix_bytes.append(&leaf_bytes);
    let leaf_hash = env.crypto().sha256(&leaf_prefix_bytes);
    let leaf_bytes_n: BytesN<32> = leaf_hash.to_bytes();
    let leaf_bytes_b: Bytes = leaf_bytes_n.to_bytes();

    let proof_prefix = BytesN::from_array(&env, &[0x01; 1]);
    let mut proof_prefix_bytes: Bytes = proof_prefix.to_bytes();
    proof_bytes.append(&leaf_bytes_b);
    proof_prefix_bytes.append(&proof_bytes);
    let proof_hash = env.crypto().sha256(&proof_prefix_bytes);
    let proof_hash_bytes_n: BytesN<32> = proof_hash.to_bytes();
    proof_hash_bytes_n
}

#[test]
fn test() {
    let env = Env::default();
    let root = initialize_root(&env);
    let client = MerkleProofClient::new(&env, &env.register(MerkleProof, (&root,)));

    let leaf = BytesN::from_array(&env, &[2; 32]);
    let proof = BytesN::from_array(&env, &[16; 32]);
    let proof_list = vec![&env, proof];
    let proof_empty: Vec<BytesN<32>> = vec![&env];

    assert_eq!(client.verify(&leaf, &proof_list, &vec![&env, true]), ());
    assert!(client
        .try_verify(&leaf, &proof_empty, &vec![&env, true])
        .is_err());
    assert!(client
        .try_verify(&leaf, &proof_list, &vec![&env, true, true])
        .is_err());
}
