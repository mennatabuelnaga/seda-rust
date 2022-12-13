use near_sdk::{json_types::U128, ONE_YOCTO};
use near_units::parse_near;
use workspaces::{operations::Function, result::ValueOrReceiptId, Account, AccountId, Contract, DevNetwork, Worker};

use crate::utils::{init, register_user, zero_fee};

#[tokio::test]
async fn test_stake_unstake() -> anyhow::Result<()> {
    let initial_balance = U128::from(parse_near!("10000 N"));
    let transfer_amount = U128::from(parse_near!("100 N"));
    let worker = workspaces::sandbox().await?;
    let (token, alice, staking_pool) = init(&worker, initial_balance).await?;

    // mint tokens to alice
    let res = token
        .call("ft_transfer")
        .args_json((alice.id(), transfer_amount, Option::<bool>::None))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await?;
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

    Ok(())
}
