use near_sdk::{
    json_types::U64,
    test_utils::{get_logs, VMContextBuilder},
    testing_env,
    VMContext,
};

use crate::{
    node_registry::{HumanReadableNode, UpdateNode},
    MainchainContract,
};

fn get_context_view() -> VMContext {
    VMContextBuilder::new().is_view(true).build()
}
fn get_context(signer_account_id: String) -> VMContext {
    VMContextBuilder::new()
        .signer_account_id(signer_account_id.parse().unwrap())
        .is_view(false)
        .build()
}
fn get_context_with_deposit(signer_account_id: String) -> VMContext {
    VMContextBuilder::new()
        .signer_account_id(signer_account_id.parse().unwrap())
        .is_view(false)
        .attached_deposit(2_180_000_000_000_000_000_000) // required for register_node()
        .build()
}
fn new_contract() -> MainchainContract {
    MainchainContract::new(
        "dao_near".to_string().try_into().unwrap(),
        "token_near".to_string().try_into().unwrap(),
    )
}

#[test]
fn register_and_get_node() {
    let mut contract = new_contract();

    // register node
    testing_env!(get_context_with_deposit("bob_near".to_string()));
    contract.register_node("0.0.0.0:8080".to_string());
    assert_eq!(get_logs(), vec!["bob_near registered node"]);
    // check owner and multi_addr
    testing_env!(get_context_view());
    assert_eq!(
        "0.0.0.0:8080".to_string(),
        contract
            .get_node("bob_near".to_string().try_into().unwrap())
            .unwrap()
            .multi_addr
    );
}

#[test]
#[should_panic(expected = "Insufficient storage, need 2140000000000000000000")]
fn register_not_enough_storage() {
    let mut contract = new_contract();

    // register node
    testing_env!(get_context("bob_near".to_string()));
    contract.register_node("0.0.0.0:8080".to_string());
}

#[test]
fn set_node_multi_addr() {
    let mut contract = new_contract();

    // register node
    testing_env!(get_context_with_deposit("bob_near".to_string()));
    contract.register_node("0.0.0.0:8080".to_string());
    assert_eq!(get_logs(), vec!["bob_near registered node"]);

    // update the multi_addr
    contract.update_node(UpdateNode::SetSocketAddress("1.1.1.1:8081".to_string()));

    // check the multi_addr after updating
    testing_env!(get_context_view());
    assert_eq!(
        "1.1.1.1:8081".to_string(),
        contract
            .get_node("bob_near".to_string().try_into().unwrap())
            .unwrap()
            .multi_addr
    );
}

#[test]
fn get_nodes() {
    let mut contract = new_contract();

    // register three nodes
    testing_env!(get_context_with_deposit("bob_near".to_string()));
    contract.register_node("0.0.0.0:8080".to_string());
    assert_eq!(get_logs(), vec!["bob_near registered node",]);
    testing_env!(get_context_with_deposit("alice_near".to_string()));
    contract.register_node("1.1.1.1:8080".to_string());
    assert_eq!(get_logs(), vec!["alice_near registered node",]);
    testing_env!(get_context_with_deposit("carol_near".to_string()));
    contract.register_node("2.2.2.2:8080".to_string());
    assert_eq!(get_logs(), vec!["carol_near registered node",]);

    // define expected nodes
    let node1 = HumanReadableNode {
        account_id:          "bob_near".to_string().try_into().unwrap(),
        balance:             0,
        multi_addr:          "0.0.0.0:8080".to_string(),
        epoch_when_eligible: U64(0),
    };
    let node2 = HumanReadableNode {
        account_id:          "alice_near".to_string().try_into().unwrap(),
        balance:             0,
        multi_addr:          "1.1.1.1:8080".to_string(),
        epoch_when_eligible: U64(0),
    };
    let node3 = HumanReadableNode {
        account_id:          "carol_near".to_string().try_into().unwrap(),
        balance:             0,
        multi_addr:          "2.2.2.2:8080".to_string(),
        epoch_when_eligible: U64(0),
    };

    // get the first node
    testing_env!(get_context_view());
    let get_node = contract.get_node("bob_near".to_string().try_into().unwrap());
    assert_eq!(get_node.unwrap(), node1);

    // check the latest 2 nodes
    let latest_2_nodes = contract.get_nodes(U64(2), U64(0));
    assert_eq!(latest_2_nodes, vec![node3.clone(), node2.clone()]);

    // check the latest 3 nodes
    let latest_3_nodes = contract.get_nodes(U64(100), U64(0));
    assert_eq!(latest_3_nodes, vec![node3.clone(), node2.clone(), node1.clone()]);

    // check offset of 1
    let latest_nodes_offset = contract.get_nodes(U64(100), U64(1));
    assert_eq!(latest_nodes_offset, vec![node2, node1]);
}
