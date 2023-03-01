use bn254::{PrivateKey, PublicKey, Signature, ECDSA};
use near_sdk::json_types::U128;
use near_units::parse_near;
use rand::distributions::{Alphanumeric, DistString};
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

pub async fn init(worker: &Worker<impl DevNetwork>, initial_balance: U128) -> (Contract, Account, Account, Contract) {
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

    // create bob account
    let bob = token_contract
        .as_account()
        .create_subaccount("bob")
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

    // alice and bob storage deposits into token contract
    register_user(&token_contract, alice.id()).await;
    register_user(&token_contract, bob.id()).await;

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

    (token_contract, alice, bob, mainchain_contract)
}

pub fn generate_bn254_key() -> (PublicKey, PrivateKey) {
    let random_hex_string = hex::encode(Alphanumeric.sample_string(&mut rand::thread_rng(), 32));
    let private_key_bytes = hex::decode(random_hex_string).unwrap();

    let private_key = PrivateKey::try_from(private_key_bytes.as_ref()).unwrap();
    let public_key = PublicKey::from_private_key(&private_key);

    (public_key, private_key)
}

pub fn bn254_sign(private_key: &PrivateKey, message: &[u8]) -> Signature {
    ECDSA::sign(message, private_key).unwrap()
}
