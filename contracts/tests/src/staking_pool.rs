use near_sdk::{json_types::U128, ONE_YOCTO};
use near_units::parse_near;

use crate::utils::{init, register_user};

#[tokio::test]
async fn test_stake_unstake() {
    let initial_balance = U128::from(parse_near!("10000 N"));
    let transfer_amount = U128::from(parse_near!("100 N"));
    let worker = workspaces::sandbox().await.unwrap();
    let (token, alice, _staking_pool) = init(&worker, initial_balance).await;

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

    // alice deposits into pool
    // let res = staking_pool
    //     .call("deposit")
    //     .args_json((alice.id(), transfer_amount, Option::<bool>::None))
    //     .max_gas()
    //     .transact()
    //     .await?;
    // println!("res: {:?}", res);
    // assert!(res.is_success());
}

#[tokio::test]
async fn simulate_transfer_call_promise_panics_for_a_full_refund() {
    let initial_balance = U128::from(parse_near!("10000 N"));
    let transfer_amount = U128::from(parse_near!("100 N"));
    let worker = workspaces::sandbox().await.unwrap();
    let (token, _, staking_pool_contract) = init(&worker, initial_balance).await;

    // defi contract must be registered as a FT account
    register_user(&token, staking_pool_contract.id()).await;

    // root invests in defi by calling `ft_transfer_call`
    let res = token
        .call("ft_transfer_call")
        .args_json((
            staking_pool_contract.id(),
            transfer_amount,
            Option::<String>::None,
            "not-a-real-function".to_string(),
        ))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    let promise_failures = res.receipt_failures();
    assert_eq!(promise_failures.len(), 1);
    let failure = promise_failures[0].clone().into_result();
    if let Err(err) = failure {
        assert!(err.to_string().contains("Execution"));
    } else {
        unreachable!();
    }

    // balances remain unchanged
    let root_balance = token
        .call("ft_balance_of")
        .args_json((token.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    let defi_balance = token
        .call("ft_balance_of")
        .args_json((staking_pool_contract.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(initial_balance, root_balance);
    assert_eq!(0, defi_balance.0);
}

#[tokio::test]
async fn simulate_transfer_call_when_called_contract_not_registered_with_ft() {
    let initial_balance = U128::from(parse_near!("10000 N"));
    let transfer_amount = U128::from(parse_near!("100 N"));
    let worker = workspaces::sandbox().await.unwrap();
    let (token, _, staking_pool_contract) = init(&worker, initial_balance).await;

    // call fails because DEFI contract is not registered as FT user
    let res = token
        .call("ft_transfer_call")
        .args_json((
            staking_pool_contract.id(),
            transfer_amount,
            Option::<String>::None,
            "deposit",
        ))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_failure());

    // balances remain unchanged
    let root_balance = token
        .call("ft_balance_of")
        .args_json((token.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    let defi_balance = token
        .call("ft_balance_of")
        .args_json((staking_pool_contract.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(initial_balance.0, root_balance.0);
    assert_eq!(0, defi_balance.0);
}
