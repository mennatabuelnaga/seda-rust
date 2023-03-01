use near_sdk::{json_types::U128, ONE_YOCTO};
use near_units::parse_near;

use crate::utils::init;

#[tokio::test]
async fn test_total_supply() {
    let initial_balance = U128::from(parse_near!("10000 N"));
    let worker = workspaces::sandbox().await.unwrap();
    let (contract, ..) = init(&worker, initial_balance).await;

    let res = contract.call("ft_total_supply").view().await.unwrap();
    assert_eq!(res.json::<U128>().unwrap(), initial_balance);
}

#[tokio::test]
async fn test_simple_transfer() {
    let initial_balance = U128::from(parse_near!("10000 N"));
    let transfer_amount = U128::from(parse_near!("100 N"));
    let worker = workspaces::sandbox().await.unwrap();
    let (contract, alice, ..) = init(&worker, initial_balance).await;

    let res = contract
        .call("ft_transfer")
        .args_json((alice.id(), transfer_amount, Option::<bool>::None))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    let root_balance = contract
        .call("ft_balance_of")
        .args_json((contract.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    let alice_balance = contract
        .call("ft_balance_of")
        .args_json((alice.id(),))
        .view()
        .await
        .unwrap()
        .json::<U128>()
        .unwrap();
    assert_eq!(initial_balance.0 - transfer_amount.0, root_balance.0);
    assert_eq!(transfer_amount.0, alice_balance.0);
}

#[tokio::test]
async fn test_close_account_empty_balance() {
    let initial_balance = U128::from(parse_near!("10000 N"));
    let worker = workspaces::sandbox().await.unwrap();
    let (contract, alice, ..) = init(&worker, initial_balance).await;

    let res = alice
        .call(contract.id(), "storage_unregister")
        .args_json((Option::<bool>::None,))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.json::<bool>().unwrap());
}

#[tokio::test]
async fn test_close_account_non_empty_balance() {
    let initial_balance = U128::from(parse_near!("10000 N"));
    let worker = workspaces::sandbox().await.unwrap();
    let (contract, ..) = init(&worker, initial_balance).await;

    let res = contract
        .call("storage_unregister")
        .args_json((Option::<bool>::None,))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await;
    assert!(format!("{:?}", res).contains("Can't unregister the account with the positive balance without force"));

    let res = contract
        .call("storage_unregister")
        .args_json((Some(false),))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await;
    assert!(format!("{:?}", res).contains("Can't unregister the account with the positive balance without force"));
}

#[tokio::test]
async fn simulate_close_account_force_non_empty_balance() {
    let initial_balance = U128::from(parse_near!(
        "10000
N"
    ));
    let worker = workspaces::sandbox().await.unwrap();
    let (contract, ..) = init(&worker, initial_balance).await;

    let res = contract
        .call("storage_unregister")
        .args_json((Some(true),))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    let res = contract.call("ft_total_supply").view().await.unwrap();
    assert_eq!(res.json::<U128>().unwrap().0, 0);
}
