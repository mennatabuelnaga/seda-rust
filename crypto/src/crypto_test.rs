use std::{fs::write, path::Path};

use bip39::{Language, Mnemonic, MnemonicType};
use bn254::ECDSA;
use ed25519_dalek::{Keypair as Ed25519DalekKeyPair, Signature, Signer};

use crate::{derive_bn254_key_pair, derive_ed25519_key_pair, Bn254KeyPair, Ed25519KeyPair};

const TEST_MNEMONIC_PATH: &str = "./seda_test_mnemonic";
fn generate_test_mnemonic() {
    if !Path::new(&TEST_MNEMONIC_PATH).exists() {
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        let phrase = mnemonic.phrase();
        write(TEST_MNEMONIC_PATH, phrase).expect("Unable to write mnemonic");
    }
}

#[test]
fn generate_bn254_pair() {
    generate_test_mnemonic();
    let bn_pair: Bn254KeyPair = derive_bn254_key_pair(TEST_MNEMONIC_PATH, 1).expect("Couldn't derive bn254 key pair");
    let msg = "awesome-seda";
    let signature = ECDSA::sign(msg, &bn_pair.private_key).expect("couldnt sign msg");
    assert!(ECDSA::verify(msg, &signature, &bn_pair.public_key).is_ok())
}

#[test]
fn generate_ed25519_pair() {
    generate_test_mnemonic();
    let ed_pair: Ed25519KeyPair =
        derive_ed25519_key_pair(TEST_MNEMONIC_PATH, 1).expect("Couldn't derive ed25519 key pair");
    let dalek_pair =
        Ed25519DalekKeyPair::from_bytes(&[ed_pair.private_key.to_bytes(), ed_pair.public_key.to_bytes()].concat())
            .expect("Couldn't convert ed25519 keypair");
    let msg: &[u8] = b"awesome-seda";
    let signature: Signature = dalek_pair.sign(msg);
    assert!(dalek_pair.verify(msg, &signature).is_ok());
}
