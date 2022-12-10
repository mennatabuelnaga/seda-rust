use seda_chain_adapters::MainChainAdapterError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error(transparent)]
    RPCError(#[from] jsonrpsee::core::Error),
    #[error(transparent)]
    MainChainError(#[from] MainChainAdapterError),
    #[error("libp2p transport error")]
    P2PTrasnport(#[from] libp2p::TransportError<std::io::Error>),
    #[error("libp2p multi addr error")]
    P2PMultiAddr(#[from] libp2p::multiaddr::Error),
}

pub type Result<T, E = NodeError> = core::result::Result<T, E>;
