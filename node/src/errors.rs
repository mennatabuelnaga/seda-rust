use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error(transparent)]
    RPCError(#[from] jsonrpsee::core::Error),
    #[error(transparent)]
    MailboxError(#[from] actix::MailboxError),
    #[error(transparent)]
    P2PError(#[from] seda_p2p::P2PAdapterError),
    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Rusqlite Error: {0}")]
    RuqliteError(#[from] rusqlite::Error),
    #[error("Chain Adapter Error: {0}")]
    ChainAdapterError(#[from] seda_chains::ChainAdapterError),
    #[error("Missing app actor address in host adapter, was the node booted?")]
    MissingAppActorAddress,
}

pub type Result<T, E = NodeError> = core::result::Result<T, E>;
