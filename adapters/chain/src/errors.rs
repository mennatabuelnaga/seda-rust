#[cfg(feature = "near")]
use near_crypto::ParseKeyError;
#[cfg(feature = "near")]
use near_jsonrpc_client::methods::broadcast_tx_async::RpcBroadcastTxAsyncError;
#[cfg(feature = "near")]
use near_primitives::account::id::ParseAccountError;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum MainChainAdapterError {
    #[error("error calling contract change method")]
    CallChangeMethod(String),

    #[error("error calling contract view method")]
    CallViewMethod,

    #[error("time limit exceeded for the transaction to be recognized")]
    BadTransactionTimestamp,

    #[error("failed to extract current nonce")]
    FailedToExtractCurrentNonce,

    #[error("could not deserialize status to string")]
    BadDeserialization(#[from] serde_json::Error),

    #[error("Bad Parameters for method `{0}`")]
    BadParams(String),

    #[cfg(feature = "near")]
    #[error("error parsing string to near secretkey")]
    ParseAccountId(#[from] ParseAccountError),

    #[cfg(feature = "near")]
    #[error("near json rpc query error")]
    JsonRpcQueryError(
        #[from] near_jsonrpc_client::errors::JsonRpcError<near_jsonrpc_client::methods::query::RpcQueryError>,
    ),

    #[cfg(feature = "near")]
    #[error("error parsing string to near AccountId")]
    ParseKey(#[from] ParseKeyError),

    #[cfg(feature = "near")]
    #[error("near json rpc tx error")]
    JsonRpcTxError(#[from] near_jsonrpc_client::errors::JsonRpcError<RpcBroadcastTxAsyncError>),

    #[error("Config error: chain_rpc_url from env var or config [main_chain] section.")]
    MissingNearServerUrlConfig,

    #[error("error serializing to vec")]
    StdIoError(#[from] std::io::Error),
}

pub type Result<T, E = MainChainAdapterError> = core::result::Result<T, E>;
