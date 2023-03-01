use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Unsupported curve, we only support ed25519 and bn254")]
    UnsupportedCurveError,
    #[error("Couldn't read mnemonic phrase from file")]
    IoError(#[from] std::io::Error),
    #[error("Couldn't convert phrase to mnemonic type")]
    PhraseConversionError(#[from] anyhow::Error),
    #[error("Couldn't derive key")]
    DerivationError(#[from] concat_kdf::Error),
}

pub type Result<T, E = CryptoError> = core::result::Result<T, E>;
