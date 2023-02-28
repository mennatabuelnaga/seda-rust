use bn254::{PrivateKey, PublicKey, ECDSA};
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
            "../../../target/wasm32-unknown-unknown/release/seda_token.wasm"
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

    // create dap account
    let dao = token_contract
        .as_account()
        .create_subaccount("dao")
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
            "../../../target/wasm32-unknown-unknown/release/seda_mainchain.wasm"
        ))
        .await
        .unwrap();
    let res = mainchain_contract
        .call("new")
        .args_json((dao.id(), token_contract.id()))
        .max_gas()
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    // mainchain contract storage deposits into token contract
    register_user(&token_contract, mainchain_contract.id()).await;

    (token_contract, alice, mainchain_contract)
}

pub fn get_public_key_and_signature(account_id: &AccountId) -> (Vec<u8>, Vec<u8>) {
    let private_key_bytes = hex::decode("471b2d4f8a717f6fee84402d209ee1d4dc15ec087b8f78322f3c24d43402669b").unwrap();
    let private_key = PrivateKey::try_from(private_key_bytes.as_ref()).unwrap();
    let public_key = PublicKey::from_private_key(&private_key).to_compressed().unwrap();

    let signature = ECDSA::sign(account_id.as_bytes(), &private_key)
        .unwrap()
        .to_compressed()
        .unwrap();
    (public_key, signature)
}
