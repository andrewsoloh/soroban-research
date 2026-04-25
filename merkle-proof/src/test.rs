#![cfg(test)]
use super::*;
use soroban_sdk::{vec, Bytes, BytesN, Env};

fn initialize_root(env: &Env) -> BytesN<32> {
    let address1 = BytesN::from_array(&env, &[2; 32]);
    let address2 = BytesN::from_array(&env, &[16; 32]);

    let bytes1: Bytes = address1.into_bytes();
    let mut bytes2: Bytes = address2.into_bytes();
    bytes2.append(&bytes1);

    let hash = env.crypto().sha256(&bytes2);
    let bytes_n: BytesN<32> = hash.to_bytes();
    bytes_n
}

#[test]
fn test() {
    let env = Env::default();
    let root = initialize_root(&env);
    let client = MerkleProofClient::new(&env, &env.register(MerkleProof, (&root,)));

    let leaf = BytesN::from_array(&env, &[2; 32]);
    let address = BytesN::from_array(&env, &[16; 32]);
    let proof = vec![&env, address];

    assert_eq!(client.verify(&leaf, &proof, &vec![&env, true]), ());
    assert!(client
        .try_verify(&leaf, &proof, &vec![&env, false])
        .is_err());
}
