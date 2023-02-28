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

use crate::utils::{get_public_key_and_signature, init};

#[tokio::test]
async fn test_deposit_withdraw() {
    let initial_balance = U128::from(parse_near!("10000 N"));
    let transfer_amount = U128::from(parse_near!("100 N"));
    let worker = workspaces::sandbox().await.unwrap();
    let (token, alice, mainchain) = init(&worker, initial_balance).await;

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
    let (alice_public_key, alice_signature) = get_public_key_and_signature(alice.id());
    let res = alice
        .call(mainchain.id(), "register_node")
        .args_json(("0.0.0.0:8080".to_string(), alice_public_key, alice_signature))
        .max_gas()
        .deposit(5_000_000_000_000_000_000_000) // doesnt work with empty fn
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
    let (token, alice, mainchain) = init(&worker, initial_balance).await;

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
    let (alice_public_key, alice_signature) = get_public_key_and_signature(alice.id());
    let res = alice
        .call(mainchain.id(), "register_node")
        .args_json(("0.0.0.0:8080".to_string(), alice_public_key, alice_signature))
        .max_gas()
        .deposit(5_000_000_000_000_000_000_000) // doesnt work with empty fn
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
    let (token, alice, mainchain) = init(&worker, initial_balance).await;

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
    let (alice_public_key, alice_signature) = get_public_key_and_signature(alice.id());
    let res = alice
        .call(mainchain.id(), "register_node")
        .args_json(("0.0.0.0:8080".to_string(), alice_public_key, alice_signature))
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
