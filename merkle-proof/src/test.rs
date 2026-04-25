#![cfg(test)]
use super::*;
use soroban_sdk::{vec, Bytes, Env};

fn initialize_root(env: &Env) -> Bytes {
    let address1 = Bytes::from_slice(&env, &[1; 102]);
    let mut address2 = Bytes::from_slice(&env, &[22; 99]);

    address2.append(&address1);
    let hash = env.crypto().sha256(&address2).to_bytes().into_bytes();
    hash
}

#[test]
fn test() {
    let env = Env::default();
    let root = initialize_root(&env);
    let client = MerkleProofClient::new(&env, &env.register(MerkleProof, (&root,)));

    let leaf = Bytes::from_slice(&env, &[1; 102]);
    let address = Bytes::from_slice(&env, &[22; 99]);
    let proof = vec![&env, address];

    assert_eq!(client.verify(&leaf, &proof, &vec![&env, true]), ());
    assert!(client
        .try_verify(&leaf, &proof, &vec![&env, false])
        .is_err());
}
