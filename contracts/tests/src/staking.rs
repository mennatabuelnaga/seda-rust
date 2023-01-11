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
async fn test_stake_unstake() {
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

    // check unstaked and staked balances
    let unstaked_balance = mainchain
        .call("get_account_unstaked_balance")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    let staked_balance = mainchain
        .call("get_account_staked_balance")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(unstaked_balance, transfer_amount);
    assert_eq!(staked_balance, U128(0));

    // alice stakes entire unstaked balance
    let res = alice
        .call(mainchain.id(), "stake")
        .args_json((transfer_amount,))
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // check unstaked and staked balances again
    let unstaked_balance = mainchain
        .call("get_account_unstaked_balance")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    let staked_balance = mainchain
        .call("get_account_staked_balance")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(unstaked_balance, U128(0));
    assert_eq!(staked_balance, transfer_amount);

    // alice unstakes entire staked balance
    let res = alice
        .call(mainchain.id(), "unstake")
        .args_json((transfer_amount,))
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // check unstaked and staked balances again
    let unstaked_balance = mainchain
        .call("get_account_unstaked_balance")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    let staked_balance = mainchain
        .call("get_account_staked_balance")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(unstaked_balance, transfer_amount);
    assert_eq!(staked_balance, U128(0));
}

#[tokio::test]
async fn test_deposit_stake_unstake() {
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

    // alice deposits and stakes into pool
    let res = alice
        .call(token.id(), "ft_transfer_call")
        .args_json((
            mainchain.id(),
            transfer_amount,
            Option::<String>::None,
            "deposit-and-stake",
        ))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // check unstaked and staked balances
    let unstaked_balance = mainchain
        .call("get_account_unstaked_balance")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    let staked_balance = mainchain
        .call("get_account_staked_balance")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(unstaked_balance, U128(0));
    assert_eq!(staked_balance, transfer_amount);

    // alice unstakes entire staked balance
    let res = alice
        .call(mainchain.id(), "unstake")
        .args_json((transfer_amount,))
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // check unstaked and staked balances again
    let unstaked_balance = mainchain
        .call("get_account_unstaked_balance")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    let staked_balance = mainchain
        .call("get_account_staked_balance")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(unstaked_balance, transfer_amount);
    assert_eq!(staked_balance, U128(0));
}

#[tokio::test]
async fn test_stake_unstake_all() {
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

    // check unstaked and staked balances
    let unstaked_balance = mainchain
        .call("get_account_unstaked_balance")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    let staked_balance = mainchain
        .call("get_account_staked_balance")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(unstaked_balance, transfer_amount);
    assert_eq!(staked_balance, U128(0));

    // alice stakes entire unstaked balance
    let res = alice
        .call(mainchain.id(), "stake_all")
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // check unstaked and staked balances again
    let unstaked_balance = mainchain
        .call("get_account_unstaked_balance")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    let staked_balance = mainchain
        .call("get_account_staked_balance")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(unstaked_balance, U128(0));
    assert_eq!(staked_balance, transfer_amount);

    // alice unstakes entire staked balance
    let res = alice
        .call(mainchain.id(), "unstake_all")
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // check unstaked and staked balances again
    let unstaked_balance = mainchain
        .call("get_account_unstaked_balance")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    let staked_balance = mainchain
        .call("get_account_staked_balance")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(unstaked_balance, transfer_amount);
    assert_eq!(staked_balance, U128(0));
}
