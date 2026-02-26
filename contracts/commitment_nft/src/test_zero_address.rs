#![cfg(test)]
extern crate std;

use soroban_sdk::{testutils::Address as _, Address, BytesN, Env};
// NOTE: If your contract is named differently (e.g. `CommitmentNft`), change these imports to match.
use crate::{CommitmentNftContract, CommitmentNftContractClient};

fn generate_zero_address(env: &Env) -> Address {
    Address::from_contract_id(&BytesN::from_array(env, &[0; 32]))
}

#[test]
#[should_panic]
fn test_nft_mint_to_zero_address_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, CommitmentNftContract);
    let client = CommitmentNftContractClient::new(&env, &contract_id);

    let zero_address = generate_zero_address(&env);

    // Attempt to mint to the zero address (Add any other required arguments your mint function needs)
    client.mint(&zero_address);
}

#[test]
#[should_panic]
fn test_nft_transfer_to_zero_address_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, CommitmentNftContract);
    let client = CommitmentNftContractClient::new(&env, &contract_id);

    let sender = Address::generate(&env);
    let zero_address = generate_zero_address(&env);

    // Setup: Mint an NFT to a valid sender first
    client.mint(&sender);
    let token_id = 1; // Adjust token ID logic based on your contract

    // Attempt to transfer the NFT to the zero address
    client.transfer(&sender, &zero_address, &token_id);
}
