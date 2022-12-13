use near_sdk::{json_types::U128, ONE_YOCTO};
use near_units::parse_near;
use staking_pool::staking::RewardFeeFraction;
use workspaces::{Account, AccountId, Contract, DevNetwork, Worker};

pub fn zero_fee() -> RewardFeeFraction {
    RewardFeeFraction {
        numerator:   0,
        denominator: 1,
    }
}

pub async fn register_user(contract: &Contract, account_id: &AccountId) -> anyhow::Result<()> {
    let res = contract
        .call("storage_deposit")
        .args_json((account_id, Option::<bool>::None))
        .max_gas()
        .deposit(near_sdk::env::storage_byte_cost() * 125)
        .transact()
        .await?;
    assert!(res.is_success());

    Ok(())
}

pub async fn init(
    worker: &Worker<impl DevNetwork>,
    initial_balance: U128,
) -> anyhow::Result<(Contract, Account, Contract)> {
    // deploy and initialize token contract
    let token_contract = worker
        .dev_deploy(include_bytes!(
            "../../target/wasm32-unknown-unknown/release/seda_token.wasm"
        ))
        .await?;
    let res = token_contract
        .call("new_default_meta")
        .args_json((token_contract.id(), initial_balance))
        .max_gas()
        .transact()
        .await?;
    assert!(res.is_success());

    // create alice account and storage deposit
    let alice = token_contract
        .as_account()
        .create_subaccount("alice")
        .initial_balance(parse_near!("10 N"))
        .transact()
        .await?
        .into_result()?;
    register_user(&token_contract, alice.id()).await?;
    let res = token_contract
        .call("storage_deposit")
        .args_json((alice.id(), Option::<bool>::None))
        .deposit(near_sdk::env::storage_byte_cost() * 125)
        .max_gas()
        .transact()
        .await?;
    assert!(res.is_success());

    // deploy and initialize staking pool contract
    let staking_pool_contract = worker
        .dev_deploy(include_bytes!(
            "../../target/wasm32-unknown-unknown/release/staking_pool.wasm"
        ))
        .await?;
    let res = staking_pool_contract
        .call("new")
        .args_json((
            "alice",
            "KuTCtARNzxZQ3YvXDeLjx83FDqxv2SdQTSbiq876zR7",
            zero_fee(),
            token_contract.id(),
        ))
        .max_gas()
        .transact()
        .await?;
    assert!(res.is_success());

    return Ok((token_contract, alice, staking_pool_contract));
}
