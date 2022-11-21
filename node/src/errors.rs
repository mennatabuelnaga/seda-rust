use seda_adapters::MainChainAdapterError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error(transparent)]
    RPCError(#[from] jsonrpsee_core::Error),
    #[error(transparent)]
    MainChainError(#[from] MainChainAdapterError),
}

pub type Result<T, E = NodeError> = core::result::Result<T, E>;
