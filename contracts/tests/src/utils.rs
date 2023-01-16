use near_sdk::json_types::U128;
use near_units::parse_near;
use workspaces::{Account, AccountId, Contract, DevNetwork, Worker};

pub async fn register_user(contract: &Contract, account_id: &AccountId) {
    let res = contract
        .call("storage_deposit")
        .args_json((account_id, Option::<bool>::None))
        .max_gas()
        .deposit(near_sdk::env::storage_byte_cost() * 125)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());
}

pub async fn init(worker: &Worker<impl DevNetwork>, initial_balance: U128) -> (Contract, Account, Contract) {
    // deploy and initialize token contract
    let token_contract = worker
        .dev_deploy(include_bytes!(
            "../../target/wasm32-unknown-unknown/release/seda_token.wasm"
        ))
        .await
        .unwrap();
    let res = token_contract
        .call("new_default_meta")
        .args_json((token_contract.id(), initial_balance))
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // create alice account
    let alice = token_contract
        .as_account()
        .create_subaccount("alice")
        .initial_balance(parse_near!("10 N"))
        .transact()
        .await
        .unwrap()
        .into_result()
        .unwrap();

    // alice storage deposits into token contract
    register_user(&token_contract, alice.id()).await;

    // deploy and initialize mainchain contract
    let mainchain_contract = worker
        .dev_deploy(include_bytes!(
            "../../target/wasm32-unknown-unknown/release/seda_mainchain.wasm"
        ))
        .await
        .unwrap();
    let res = mainchain_contract
        .call("new")
        .args_json((token_contract.id(),))
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // mainchain contract storage deposits into token contract
    register_user(&token_contract, mainchain_contract.id()).await;

    (token_contract, alice, mainchain_contract)
}
