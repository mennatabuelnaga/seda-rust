use near_crypto::ParseKeyError;
use near_primitives::account::id::ParseAccountError;
use seda_adapters::MainChainAdapterError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("environment variable `{0}` is not set")]
    MissingEnvVar(String),
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
    #[error("Failed to read/write config file: {0}")]
    ConfigIoError(#[from] std::io::Error),
    #[error("Invalid Toml Conversion: {0}")]
    InvalidTomlConfig(String),
}
pub type Result<T, E = CliError> = core::result::Result<T, E>;
