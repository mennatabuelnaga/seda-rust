use bn254::{PrivateKey, PublicKey, ECDSA};
use near_sdk::{test_utils::VMContextBuilder, testing_env, VMContext};

use crate::MainchainContract;

fn get_context(is_view: bool) -> VMContext {
    VMContextBuilder::new()
        .signer_account_id("bob_near".parse().unwrap())
        .is_view(is_view)
        .build()
}

fn new_contract() -> MainchainContract {
    MainchainContract::new(
        "dao_near".to_string().try_into().unwrap(),
        "seda_token".to_string().try_into().unwrap(),
    )
}

/// Test `ECDSA::verify` function with own signed message
#[test]
fn test_verify_signed_msg() {
    let context = get_context(false);
    testing_env!(context);
    let mut contract = new_contract();

    // Public key
    let private_key = hex::decode("2009da7287c158b126123c113d1c85241b6e3294dd75c643588630a8bc0f934c").unwrap();
    let private_key = PrivateKey::try_from(private_key.as_ref()).unwrap();
    let public_key = PublicKey::from_private_key(&private_key).to_compressed().unwrap();

    // Signature
    let signature_vec = hex::decode("020f047a153e94b5f109e4013d1bd078112817cf0d58cdf6ba8891f9849852ba5b").unwrap();

    // Message signed
    let msg = hex::decode("73616d706c65").unwrap();

    // Verify signature
    assert!(
        contract.bn254_verify(msg, signature_vec, public_key) == true,
        "Verification failed"
    );
}

/// Test aggregate signature verification
#[test]
fn test_verify_aggregate_signatures() {
    let context = get_context(false);
    testing_env!(context);
    let mut contract = new_contract();

    // Message
    let msg = hex::decode("73616d706c65").unwrap();

    // Signature 1
    let private_key_1_bytes = hex::decode("1ab1126ff2e37c6e6eddea943ccb3a48f83b380b856424ee552e113595525565").unwrap();
    let private_key_1 = PrivateKey::try_from(private_key_1_bytes.as_ref()).unwrap();
    let sign_1 = ECDSA::sign(&msg, &private_key_1).unwrap();
    let sign_1_bytes = sign_1.to_compressed().unwrap();

    let public_key_1 = PublicKey::from_private_key(&private_key_1);
    let public_key_1_bytes = public_key_1.to_compressed().unwrap();

    // Signature 2
    let secret_key_2_bytes = hex::decode("2009da7287c158b126123c113d1c85241b6e3294dd75c643588630a8bc0f934c").unwrap();
    let private_key_2 = PrivateKey::try_from(secret_key_2_bytes.as_ref()).unwrap();
    let sign_2 = ECDSA::sign(&msg, &private_key_2).unwrap();
    let sign_2_bytes = sign_2.to_compressed().unwrap();

    let public_key_2 = PublicKey::from_private_key(&private_key_2);
    let public_key_2_bytes = public_key_2.to_compressed().unwrap();

    // Public Key and Signature aggregation
    let agg_public_key = (public_key_1 + public_key_2).to_compressed().unwrap();
    let agg_signature = (sign_1 + sign_2).to_compressed().unwrap();

    // Verification single signatures
    assert!(
        contract.bn254_verify(msg.clone(), sign_1_bytes, public_key_1_bytes) == true,
        "Signature 1 verification failed"
    );
    assert!(
        contract.bn254_verify(msg.clone(), sign_2_bytes, public_key_2_bytes) == true,
        "Signature 2 signature verification failed"
    );

    // Aggregate signature verification
    assert!(
        contract.bn254_verify(msg, agg_signature, agg_public_key) == true,
        "Aggregated signature verification failed"
    );
}
