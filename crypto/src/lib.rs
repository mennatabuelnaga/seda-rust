use std::{
    fs::{read_to_string, File},
    io::Write,
};

use bip39::{Language, Mnemonic, MnemonicType, Seed};
use bn254::{PrivateKey as Bn254PrivateKey, PublicKey as Bn254PublicKey};
use concat_kdf::derive_key;
use ed25519_dalek::{PublicKey as Ed25519PublicKey, SecretKey as Ed25519PrivateKey, SECRET_KEY_LENGTH};
use serde::{Deserialize, Serialize};
mod errors;
use errors::CryptoError;

#[derive(PartialEq)]
pub enum KeyType {
    Ed25519,
    Bn254,
}

pub enum PrivateKey {
    Ed25519(Ed25519PrivateKey),
    Bn254(Bn254PrivateKey),
}

pub enum PublicKey {
    Ed25519(Ed25519PublicKey),
    Bn254(Bn254PublicKey),
}

pub struct KeyPair {
    public_key:  PublicKey,
    private_key: PrivateKey,
}

pub fn derive_key_pair(key_type: KeyType, index: usize) -> Result<KeyPair, CryptoError> {
    let phrase = read_to_string("./seda_mnemonic")?;
    let mnemonic = Mnemonic::from_phrase(&phrase, Language::English)?;
    let seed = Seed::new(&mnemonic, "seda");

    if key_type == KeyType::Ed25519 {
        let master_sk = derive_key::<sha2::Sha256>(seed.as_bytes(), b"ed25519", SECRET_KEY_LENGTH)?;
        let sk = derive_key::<sha2::Sha256>(master_sk.as_slice(), &index.to_ne_bytes(), SECRET_KEY_LENGTH)?;
        let private_key = Ed25519PrivateKey::from_bytes(sk.as_slice()).unwrap();
        let public_key: Ed25519PublicKey = (&private_key).into();
        Ok(KeyPair {
            public_key:  PublicKey::Ed25519(public_key),
            private_key: PrivateKey::Ed25519(private_key),
        })
    } else if key_type == KeyType::Bn254 {
        let master_sk = derive_key::<sha2::Sha256>(seed.as_bytes(), b"bn254", SECRET_KEY_LENGTH)?;
        let sk = derive_key::<sha2::Sha256>(master_sk.as_slice(), &index.to_ne_bytes(), SECRET_KEY_LENGTH)?;
        let private_key = Bn254PrivateKey::try_from(sk.as_slice()).unwrap();
        let public_key = Bn254PublicKey::from_private_key(&private_key);
        Ok(KeyPair {
            public_key:  PublicKey::Bn254(public_key),
            private_key: PrivateKey::Bn254(private_key),
        })
    } else {
        Err(CryptoError::UnsupportedCurveError)
    }
}
