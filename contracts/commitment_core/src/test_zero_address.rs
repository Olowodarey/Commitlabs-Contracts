#![cfg(test)]
extern crate std;

use soroban_sdk::{testutils::Address as _, Address, BytesN, Env};
// NOTE: Change these if your struct is named something like `CommitmentCore` instead
use crate::{CommitmentCoreContract, CommitmentCoreContractClient};

fn generate_zero_address(env: &Env) -> Address {
    Address::from_contract_id(&BytesN::from_array(env, &[0; 32]))
}

#[test]
#[should_panic]
fn test_create_commitment_zero_owner_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, CommitmentCoreContract);
    let client = CommitmentCoreContractClient::new(&env, &contract_id);

    let zero_owner = generate_zero_address(&env);

    // Attempt to create a commitment with the zero address
    // (Add the remaining arguments required by your create_commitment function)
    client.create_commitment(&zero_owner);
}
