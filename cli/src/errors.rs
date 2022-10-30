use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("environment variable `{0}` is not set")]
    MissingEnvVar(String),
    #[error("failed to extract current nonce")]
    FailedToExtractCurrentNonce,
    #[error("json rpc error")]
    JsonRpcError(#[from] near_jsonrpc_client::errors::JsonRpcError<near_jsonrpc_client::methods::query::RpcQueryError>),
}

pub type Result<T, E = CliError> = core::result::Result<T, E>;

pub fn get_env_var(var_name: &str) -> Result<String, CliError> {
    match std::env::var(var_name) {
        Ok(val) => Ok(val),
        Err(_) => Err(CliError::MissingEnvVar(var_name.to_string())),
    }
}
