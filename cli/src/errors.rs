use near_crypto::ParseKeyError;
use near_primitives::account::id::ParseAccountError;
use seda_adapters::MainChainAdapterError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TomlError {
    #[error("Invalid Toml Deserialization: {0}")]
    Deserialize(#[from] toml::de::Error),
    #[error("Invalid Toml Serialization: {0}")]
    Serialize(#[from] toml::ser::Error),
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error("environment variable `{0}` is not set")]
    MissingEnvVar(String),
    // #[error("failed to extract current nonce")]
    // FailedToExtractCurrentNonce,
    #[error("near json rpc error")]
    JsonRpcError(#[from] near_jsonrpc_client::errors::JsonRpcError<near_jsonrpc_client::methods::query::RpcQueryError>),
    #[error("jsonrpsee client error")]
    JsonRpcClientError(#[from] jsonrpsee::core::error::Error),
    #[error("error parsing string to near AccountId")]
    ParseAccountId(#[from] ParseAccountError),
    #[error("error parsing string to near AccountId")]
    ParseKey(#[from] ParseKeyError),
    #[error(transparent)]
    MainChainAdapterError(#[from] MainChainAdapterError),
    #[error("Config io error: {0}")]
    ConfigIoError(#[from] std::io::Error),
    #[error("Config error: {0}")]
    ConfigError(String),
    #[error(transparent)]
    InvalidTomlConfig(#[from] TomlError),
}

impl From<&str> for CliError {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl From<String> for CliError {
    fn from(value: String) -> Self {
        Self::ConfigError(value)
    }
}

pub type Result<T, E = CliError> = core::result::Result<T, E>;
