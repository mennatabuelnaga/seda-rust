use near_crypto::ParseKeyError;
use near_primitives::account::id::ParseAccountError;
use thiserror::Error;

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
}

pub type Result<T, E = CliError> = core::result::Result<T, E>;

pub fn get_env_var(var_name: &str) -> Result<String, CliError> {
    match std::env::var(var_name) {
        Ok(val) => Ok(val),
        Err(_) => Err(CliError::MissingEnvVar(var_name.to_string())),
    }
}
