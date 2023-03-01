use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    json_types::{U128, U64},
    serde::{Deserialize, Serialize},
    AccountId,
    Balance,
    ONE_YOCTO,
};
use near_units::parse_near;
use seda_mainchain::node_registry::HumanReadableNode;

use crate::utils::{bn254_sign, generate_bn254_key, init};

#[tokio::test]
async fn test_deposit_withdraw() {
    let initial_balance = U128::from(parse_near!("10000 N"));
    let transfer_amount = U128::from(parse_near!("100 N"));
    let worker = workspaces::sandbox().await.unwrap();
    let (token, alice, _, mainchain) = init(&worker, initial_balance).await;

    // transfer some tokens to alice
    let res = token
        .call("ft_transfer")
        .args_json((alice.id(), transfer_amount, Option::<bool>::None))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());
    let alice_initial_balance = alice
        .call(token.id(), "ft_balance_of")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    // alice registers node
    let (alice_public_key, alice_private_key) = generate_bn254_key();
    let alice_signature = bn254_sign(&alice_private_key, alice.id().as_bytes());
    let res = alice
        .call(mainchain.id(), "register_node")
        .args_json((
            "0.0.0.0:8080".to_string(),
            alice_public_key.to_compressed().unwrap(),
            alice_signature.to_compressed().unwrap(),
        ))
        .max_gas()
        .deposit(5_000_000_000_000_000_000_000)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // alice deposits into pool (without staking)
    let res = alice
        .call(token.id(), "ft_transfer_call")
        .args_json((mainchain.id(), transfer_amount, Option::<String>::None, "deposit"))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // check if alice's balance has decreased by `transfer_amount`
    let alice_balance_after_deposit = alice
        .call(token.id(), "ft_balance_of")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(
        alice_balance_after_deposit,
        U128(alice_initial_balance.0 - transfer_amount.0)
    );

    // check alice's deposited amount
    let unstaked_balance = mainchain
        .call("get_node_balance")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(unstaked_balance, transfer_amount);

    // alice withdraws
    let res = alice
        .call(mainchain.id(), "withdraw")
        .args_json((transfer_amount,))
        .max_gas()
        .deposit(0)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // check if alice's balance is now `alice_initial_balance` again
    let alice_balance_after_withdraw = alice
        .call(token.id(), "ft_balance_of")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(alice_balance_after_withdraw, alice_initial_balance);
}

#[tokio::test]
async fn test_deposit_withdraw_all() {
    let initial_balance = U128::from(parse_near!("10000 N"));
    let transfer_amount = U128::from(parse_near!("100 N"));
    let worker = workspaces::sandbox().await.unwrap();
    let (token, alice, _, mainchain) = init(&worker, initial_balance).await;

    // transfer some tokens to alice
    let res = token
        .call("ft_transfer")
        .args_json((alice.id(), transfer_amount, Option::<bool>::None))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());
    let alice_initial_balance = alice
        .call(token.id(), "ft_balance_of")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    // alice registers node
    let (alice_public_key, alice_private_key) = generate_bn254_key();
    let alice_signature = bn254_sign(&alice_private_key, alice.id().as_bytes());
    let res = alice
        .call(mainchain.id(), "register_node")
        .args_json((
            "0.0.0.0:8080".to_string(),
            alice_public_key.to_compressed().unwrap(),
            alice_signature.to_compressed().unwrap(),
        ))
        .max_gas()
        .deposit(5_000_000_000_000_000_000_000)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // alice deposits into pool (without staking)
    let res = alice
        .call(token.id(), "ft_transfer_call")
        .args_json((mainchain.id(), transfer_amount, Option::<String>::None, "deposit"))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // check if alice's balance has decreased by `transfer_amount`
    let alice_balance_after_deposit = alice
        .call(token.id(), "ft_balance_of")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(
        alice_balance_after_deposit,
        U128(alice_initial_balance.0 - transfer_amount.0)
    );

    // alice withdraws all
    let res = alice
        .call(mainchain.id(), "withdraw_all")
        .max_gas()
        .deposit(0)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // check if alice's balance is now `alice_initial_balance` again
    let alice_balance_after_withdraw = alice
        .call(token.id(), "ft_balance_of")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(alice_balance_after_withdraw, alice_initial_balance);
}

#[tokio::test]
async fn test_is_node_active() {
    let initial_balance = U128::from(parse_near!("10000 N"));
    let transfer_amount = U128::from(parse_near!("100 N"));
    let worker = workspaces::sandbox().await.unwrap();
    let (token, alice, _, mainchain) = init(&worker, initial_balance).await;

    // transfer some tokens to alice
    let res = token
        .call("ft_transfer")
        .args_json((alice.id(), transfer_amount, Option::<bool>::None))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // alice registers node
    let (alice_public_key, alice_private_key) = generate_bn254_key();
    let alice_signature = bn254_sign(&alice_private_key, alice.id().as_bytes());
    let res = alice
        .call(mainchain.id(), "register_node")
        .args_json((
            "0.0.0.0:8080".to_string(),
            alice_public_key.to_compressed().unwrap(),
            alice_signature.to_compressed().unwrap(),
        ))
        .max_gas()
        .deposit(5_000_000_000_000_000_000_000)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // alice deposits into contract
    let res = alice
        .call(token.id(), "ft_transfer_call")
        .args_json((mainchain.id(), transfer_amount, Option::<String>::None, "deposit"))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // check alice's deposited amount
    let balance = mainchain
        .call("get_node_balance")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(balance, transfer_amount);

    // check alice is not eligible to propose
    let is_node_active = mainchain
        .call("is_node_active")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<bool>()
        .unwrap();
    assert!(!is_node_active);

    let get_epoch = mainchain
        .call("get_current_epoch")
        .view()
        .await
        .unwrap()
        .json::<u64>()
        .unwrap();
    println!("current epoch: {}", get_epoch);

    // time travel forward in time to a future epoch
    let blocks_to_advance = 2000;
    worker.fast_forward(blocks_to_advance).await.unwrap();

    let get_epoch = mainchain
        .call("get_current_epoch")
        .view()
        .await
        .unwrap()
        .json::<u64>()
        .unwrap();
    println!("epoch after time travel: {}", get_epoch);

    let node = mainchain
        .call("get_node")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<HumanReadableNode>()
        .unwrap();
    println!("node: {:?}", node);

    // check is_node_active
    let is_node_active = mainchain
        .call("is_node_active")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<bool>()
        .unwrap();
    assert!(is_node_active); // future epoch so should be true

    // alice withdraws all
    let res = alice
        .call(mainchain.id(), "withdraw_all")
        .max_gas()
        .deposit(0)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // assert alice is now not eligible to propose (not enough deposited)
    let is_node_active = mainchain
        .call("is_node_active")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<bool>()
        .unwrap();
    assert!(!is_node_active);
}

#[tokio::test]
async fn test_post_signed_batch() {
    let initial_balance = U128::from(parse_near!("10000 N"));
    let transfer_amount = U128::from(parse_near!("100 N"));
    let worker = workspaces::sandbox().await.unwrap();
    let (token, alice, bob, mainchain) = init(&worker, initial_balance).await;

    // post some data requests to the accumulator
    let res = alice
        .call(mainchain.id(), "post_data_request")
        .args_json(("data_request_1".to_string(),))
        .max_gas()
        .deposit(2_140_000_000_000_000_000_000)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());
    let res = alice
        .call(mainchain.id(), "post_data_request")
        .args_json(("data_request_2".to_string(),))
        .max_gas()
        .deposit(2_140_000_000_000_000_000_000)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());
    let res = alice
        .call(mainchain.id(), "post_data_request")
        .args_json(("data_request_3".to_string(),))
        .max_gas()
        .deposit(2_140_000_000_000_000_000_000)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // get the merkle root (for all nodes to sign)
    let merkle_root = mainchain
        .call("compute_merkle_root")
        .args_json(())
        .view()
        .await
        .unwrap()
        .json::<Vec<u8>>()
        .unwrap();

    // transfer some tokens to alice and bob
    let res = token
        .call("ft_transfer")
        .args_json((alice.id(), transfer_amount, Option::<bool>::None))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());
    let res = token
        .call("ft_transfer")
        .args_json((bob.id(), transfer_amount, Option::<bool>::None))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // alice and bob register node
    let (alice_public_key, alice_private_key) = generate_bn254_key();
    let (bob_public_key, bob_private_key) = generate_bn254_key();
    let alice_signature = bn254_sign(&alice_private_key, alice.id().as_bytes());
    let bob_signature = bn254_sign(&bob_private_key, bob.id().as_bytes());
    let res = alice
        .call(mainchain.id(), "register_node")
        .args_json((
            "0.0.0.0:8080".to_string(),
            &alice_public_key.to_compressed().unwrap(),
            alice_signature.to_compressed().unwrap(),
        ))
        .max_gas()
        .deposit(5_000_000_000_000_000_000_000)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());
    let res = bob
        .call(mainchain.id(), "register_node")
        .args_json((
            "1.1.1.1:8080".to_string(),
            &bob_public_key.to_compressed().unwrap(),
            bob_signature.to_compressed().unwrap(),
        ))
        .max_gas()
        .deposit(5_000_000_000_000_000_000_000)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // alice and bob deposits into contract
    let res = alice
        .call(token.id(), "ft_transfer_call")
        .args_json((mainchain.id(), transfer_amount, Option::<String>::None, "deposit"))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());
    let res = bob
        .call(token.id(), "ft_transfer_call")
        .args_json((mainchain.id(), transfer_amount, Option::<String>::None, "deposit"))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // time travel forward in time to a future epoch
    let blocks_to_advance = 2000;
    worker.fast_forward(blocks_to_advance).await.unwrap();

    // alice signs the merkle root
    let alice_merkle_root_signature = bn254_sign(&alice_private_key, &merkle_root);
    let bob_merkle_root_signature = bn254_sign(&bob_private_key, &merkle_root);

    // aggregate the signatures
    let agg_public_key = alice_public_key + bob_public_key;
    let agg_signature = alice_merkle_root_signature + bob_merkle_root_signature;

    // alice posts the signed batch
    let res = alice
        .call(mainchain.id(), "post_signed_batch")
        .args_json((
            agg_signature.to_compressed().unwrap(),
            agg_public_key.to_compressed().unwrap(),
            [alice.id(), bob.id()],
        ))
        .max_gas()
        .deposit(510_000_000_000_000_000_000)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());
}
