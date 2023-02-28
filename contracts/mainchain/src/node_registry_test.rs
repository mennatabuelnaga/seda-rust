use bn254::{PrivateKey, PublicKey, ECDSA};
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
        .attached_deposit(4_110_000_000_000_000_000_000) // required for register_node()
        .build()
}
fn new_contract() -> MainchainContract {
    MainchainContract::new(
        "dao_near".to_string().try_into().unwrap(),
        "token_near".to_string().try_into().unwrap(),
    )
}
fn get_bob_key_and_signature() -> (Vec<u8>, Vec<u8>) {
    let bob_private_key_bytes =
        hex::decode("1ab1126ff2e37c6e6eddea943ccb3a48f83b380b856424ee552e113595525565").unwrap();
    let bob_private_key = PrivateKey::try_from(bob_private_key_bytes.as_ref()).unwrap();
    let bob_public_key = PublicKey::from_private_key(&bob_private_key).to_compressed().unwrap();

    let msg = "bob_near".as_bytes();
    let bob_signature = ECDSA::sign(msg, &bob_private_key).unwrap().to_compressed().unwrap();
    (bob_public_key, bob_signature)
}

fn get_alice_duplicated_key() -> (Vec<u8>, Vec<u8>) {
    let bob_private_key_bytes =
        hex::decode("1ab1126ff2e37c6e6eddea943ccb3a48f83b380b856424ee552e113595525565").unwrap();
    let bob_private_key = PrivateKey::try_from(bob_private_key_bytes.as_ref()).unwrap();
    let bob_public_key = PublicKey::from_private_key(&bob_private_key).to_compressed().unwrap();

    let msg = "alice_near".as_bytes();
    let alice_signature = ECDSA::sign(msg, &bob_private_key).unwrap().to_compressed().unwrap();
    (bob_public_key, alice_signature)
}

fn get_alice_key_and_signature() -> (Vec<u8>, Vec<u8>) {
    let alice_private_key_bytes =
        hex::decode("471b2d4f8a717f6fee84402d209ee1d4dc15ec087b8f78322f3c24d43402669b").unwrap();
    let alice_private_key = PrivateKey::try_from(alice_private_key_bytes.as_ref()).unwrap();
    let alice_public_key = PublicKey::from_private_key(&alice_private_key).to_compressed().unwrap();

    let msg = "alice_near".as_bytes();
    let alice_signature = ECDSA::sign(msg, &alice_private_key).unwrap().to_compressed().unwrap();
    (alice_public_key, alice_signature)
}

fn get_carol_key_and_signature() -> (Vec<u8>, Vec<u8>) {
    let carol_private_key_bytes =
        hex::decode("f99091aaf6856d79776faa15ecc647b937cf5ea94c9b9dab9fd8bc6c38359099").unwrap();
    let carol_private_key = PrivateKey::try_from(carol_private_key_bytes.as_ref()).unwrap();
    let carol_public_key = PublicKey::from_private_key(&carol_private_key).to_compressed().unwrap();

    let msg = "carol_near".as_bytes();
    let carol_signature = ECDSA::sign(msg, &carol_private_key).unwrap().to_compressed().unwrap();
    (carol_public_key, carol_signature)
}

#[test]
fn register_and_get_node() {
    let mut contract = new_contract();
    let (bob_public_key, bob_signature) = get_bob_key_and_signature();

    // register node
    testing_env!(get_context_with_deposit("bob_near".to_string()));
    contract.register_node("0.0.0.0:8080".to_string(), bob_public_key, bob_signature);
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
#[should_panic(expected = "Insufficient storage, need 4050000000000000000000")]
fn register_not_enough_storage() {
    let mut contract = new_contract();
    let (bob_public_key, bob_signature) = get_bob_key_and_signature();

    // register node
    testing_env!(get_context("bob_near".to_string()));
    contract.register_node("0.0.0.0:8080".to_string(), bob_public_key, bob_signature);
}

#[test]
fn set_node_multi_addr() {
    let mut contract = new_contract();
    let (bob_public_key, bob_signature) = get_bob_key_and_signature();

    // register node
    testing_env!(get_context_with_deposit("bob_near".to_string()));
    contract.register_node("0.0.0.0:8080".to_string(), bob_public_key, bob_signature);
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
    let (bob_public_key, bob_signature) = get_bob_key_and_signature();
    let (alice_public_key, alice_signature) = get_alice_key_and_signature();
    let (carol_public_key, carol_signature) = get_carol_key_and_signature();

    // register three nodes
    testing_env!(get_context_with_deposit("bob_near".to_string()));
    contract.register_node("0.0.0.0:8080".to_string(), bob_public_key.clone(), bob_signature);
    assert_eq!(get_logs(), vec!["bob_near registered node",]);
    testing_env!(get_context_with_deposit("alice_near".to_string()));
    contract.register_node("1.1.1.1:8080".to_string(), alice_public_key.clone(), alice_signature);
    assert_eq!(get_logs(), vec!["alice_near registered node",]);
    testing_env!(get_context_with_deposit("carol_near".to_string()));
    contract.register_node("2.2.2.2:8080".to_string(), carol_public_key.clone(), carol_signature);
    assert_eq!(get_logs(), vec!["carol_near registered node",]);

    // define expected nodes
    let node1 = HumanReadableNode {
        account_id:          "bob_near".to_string().try_into().unwrap(),
        balance:             0,
        multi_addr:          "0.0.0.0:8080".to_string(),
        epoch_when_eligible: U64(0),
        bn254_public_key:    bob_public_key,
    };
    let node2 = HumanReadableNode {
        account_id:          "alice_near".to_string().try_into().unwrap(),
        balance:             0,
        multi_addr:          "1.1.1.1:8080".to_string(),
        epoch_when_eligible: U64(0),
        bn254_public_key:    alice_public_key,
    };
    let node3 = HumanReadableNode {
        account_id:          "carol_near".to_string().try_into().unwrap(),
        balance:             0,
        multi_addr:          "2.2.2.2:8080".to_string(),
        epoch_when_eligible: U64(0),
        bn254_public_key:    carol_public_key,
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
    assert_eq!(latest_3_nodes, vec![node3, node2.clone(), node1.clone()]);

    // check offset of 1
    let latest_nodes_offset = contract.get_nodes(U64(100), U64(1));
    assert_eq!(latest_nodes_offset, vec![node2, node1]);
}

#[test]
#[should_panic(expected = "bn254_public_key already exists")]
fn duplicated_key() {
    let mut contract = new_contract();
    let (bob_public_key, bob_signature) = get_bob_key_and_signature();
    let (alice_public_key, alice_signature) = get_alice_duplicated_key();

    // bob registers node
    testing_env!(get_context_with_deposit("bob_near".to_string()));
    contract.register_node("0.0.0.0:8080".to_string(), bob_public_key, bob_signature);

    // alice registers node with duplicated key
    testing_env!(get_context_with_deposit("alice_near".to_string()));
    contract.register_node("1.1.1.1:8080".to_string(), alice_public_key, alice_signature);
}
