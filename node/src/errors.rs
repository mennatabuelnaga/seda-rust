use actix::MailboxError;
use seda_p2p_adapters::P2PAdapterError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error(transparent)]
    RPCError(#[from] jsonrpsee::core::Error),
    #[error(transparent)]
    MailboxError(#[from] MailboxError),
    #[error(transparent)]
    P2PError(#[from] P2PAdapterError),
}

pub type Result<T, E = NodeError> = core::result::Result<T, E>;
