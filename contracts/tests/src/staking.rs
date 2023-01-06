use near_sdk::{json_types::U128, ONE_YOCTO};
use near_units::parse_near;

use crate::utils::init;

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

    // alice deposits into pool
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

    // alice withdraws
    // let res = alice
    //     .call(mainchain.id(), "withdraw")
    //     .args_json((transfer_amount,))
    //     .max_gas()
    //     .deposit(ONE_YOCTO)
    //     .transact()
    //     .await
    //     .unwrap();
    // assert!(res.is_success());

    // // check if alice's balance is now `alice_initial_balance` again
    // let alice_balance_after_withdraw = alice
    //     .call(token.id(), "ft_balance_of")
    //     .args_json((alice.id(),))
    //     .view()
    //     .await
    //     .unwrap()
    //     .json::<U128>()
    //     .unwrap();
    // assert_eq!(alice_balance_after_withdraw, alice_initial_balance);
}

// #[tokio::test]
// async fn test_stake() {
//     let initial_balance = U128::from(parse_near!("10000 N"));
//     let transfer_amount = U128::from(parse_near!("100 N"));
//     let worker = workspaces::sandbox().await.unwrap();
//     let (token, alice, mainchain) = init(&worker, initial_balance).await;

//     // transfer some tokens to alice
//     let res = token
//         .call("ft_transfer")
//         .args_json((alice.id(), transfer_amount, Option::<bool>::None))
//         .max_gas()
//         .deposit(ONE_YOCTO)
//         .transact()
//         .await
//         .unwrap();
//     assert!(res.is_success());

//     // alice deposits into pool
//     let res = alice
//         .call(token.id(), "ft_transfer_call")
//         .args_json((mainchain.id(), transfer_amount, Option::<String>::None,
// "deposit"))         .max_gas()
//         .deposit(ONE_YOCTO)
//         .transact()
//         .await
//         .unwrap();
//     assert!(res.is_success());

//     // check unstaked and staked balances
//     let unstaked_balance = mainchain
//         .call("get_account_unstaked_balance")
//         .args_json((alice.id(),))
//         .view()
//         .await
//         .unwrap()
//         .json::<U128>()
//         .unwrap();
//     let staked_balance = mainchain
//         .call("get_account_staked_balance")
//         .args_json((alice.id(),))
//         .view()
//         .await
//         .unwrap()
//         .json::<U128>()
//         .unwrap();
//     assert_eq!(unstaked_balance, transfer_amount);
//     assert_eq!(staked_balance, U128(0));

//     // alice stakes entire unstaked balance
//     let res = alice
//         .call(mainchain.id(), "stake")
//         .args_json((transfer_amount,))
//         .max_gas()
//         .transact()
//         .await
//         .unwrap();
//     assert!(res.is_success());

//     // check unstaked and staked balances again
//     let unstaked_balance = mainchain
//         .call("get_account_unstaked_balance")
//         .args_json((alice.id(),))
//         .view()
//         .await
//         .unwrap()
//         .json::<U128>()
//         .unwrap();
//     let staked_balance = mainchain
//         .call("get_account_staked_balance")
//         .args_json((alice.id(),))
//         .view()
//         .await
//         .unwrap()
//         .json::<U128>()
//         .unwrap();
//     assert_eq!(unstaked_balance, U128(0));
//     assert_eq!(staked_balance, transfer_amount);
// }

// #[tokio::test]
// async fn simulate_transfer_call_promise_panics_for_a_full_refund() {
//     let initial_balance = U128::from(parse_near!("10000 N"));
//     let transfer_amount = U128::from(parse_near!("100 N"));
//     let worker = workspaces::sandbox().await.unwrap();
//     let (token, _, mainchain_contract) = init(&worker,
// initial_balance).await;

//     // defi contract must be registered as a FT account
//     register_user(&token, mainchain_contract.id()).await;

//     // root invests in defi by calling `ft_transfer_call`
//     let res = token
//         .call("ft_transfer_call")
//         .args_json((
//             mainchain_contract.id(),
//             transfer_amount,
//             Option::<String>::None,
//             "not-a-real-function".to_string(),
//         ))
//         .max_gas()
//         .deposit(ONE_YOCTO)
//         .transact()
//         .await
//         .unwrap();
//     assert!(res.is_success());

//     let promise_failures = res.receipt_failures();
//     assert_eq!(promise_failures.len(), 1);
//     let failure = promise_failures[0].clone().into_result();
//     if let Err(err) = failure {
//         assert!(err.to_string().contains("Execution"));
//     } else {
//         unreachable!();
//     }

//     // balances remain unchanged
//     let root_balance = token
//         .call("ft_balance_of")
//         .args_json((token.id(),))
//         .view()
//         .await
//         .unwrap()
//         .json::<U128>()
//         .unwrap();
//     let defi_balance = token
//         .call("ft_balance_of")
//         .args_json((mainchain_contract.id(),))
//         .view()
//         .await
//         .unwrap()
//         .json::<U128>()
//         .unwrap();
//     assert_eq!(initial_balance, root_balance);
//     assert_eq!(0, defi_balance.0);
// }
