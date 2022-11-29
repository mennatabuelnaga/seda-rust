use seda_chain_adapters::MainChainAdapterError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error(transparent)]
    RPCError(#[from] jsonrpsee::core::Error),
    #[error(transparent)]
    MainChainError(#[from] MainChainAdapterError),
}

pub type Result<T, E = NodeError> = core::result::Result<T, E>;
