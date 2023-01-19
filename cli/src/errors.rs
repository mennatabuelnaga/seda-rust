use near_crypto::ParseKeyError;
use near_primitives::account::id::ParseAccountError;
use seda_chains::ChainAdapterError;
use seda_config::ConfigError;
use seda_node::NodeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("near json rpc error")]
    JsonRpcError(#[from] near_jsonrpc_client::errors::JsonRpcError<near_jsonrpc_client::methods::query::RpcQueryError>),
    #[error("jsonrpsee client error")]
    JsonRpcClientError(#[from] jsonrpsee::core::error::Error),
    #[error("error parsing string to near AccountId")]
    ParseAccountId(#[from] ParseAccountError),
    #[error("error parsing string to near AccountId")]
    ParseKey(#[from] ParseKeyError),
    #[error(transparent)]
    ChainAdapterError(#[from] ChainAdapterError),
    #[error("Config error: {0}")]
    LoadConfigError(#[from] ConfigError),
    #[error("Config error: {0}")]
    ConfigError(String),
    #[error(transparent)]
    NodeError(#[from] NodeError),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[cfg(debug_assertions)]
    #[error(transparent)]
    CLIDocumentError(#[from] std::io::Error),
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
