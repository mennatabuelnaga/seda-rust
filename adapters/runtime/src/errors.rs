use std::{array::TryFromSliceError, str::Utf8Error, num::ParseIntError};

use seda_chain_adapters::MainChainAdapterError;
use thiserror::Error;
use actix::MailboxError;
#[derive(Debug, Error)]
pub enum RuntimeAdapterError {
    #[error("{0:?}")]
    StringBytesConversion(#[from] Utf8Error),
    #[error("{0}")]
    NumBytesConversion(#[from] TryFromSliceError),

    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Rusqlite Error: {0}")]
    RuqliteError(#[from] rusqlite::Error),
    #[error("Chain Interactions Error: {0}")]
    ChainInteractionsError(String),

    #[error("Mailbox Error Error: {0}")]
    MailboxError(#[from] MailboxError),

    #[error("Parse Integer Error: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("MainChain Adapter Error: {0}")]
    MainChainAdapterError(#[from] MainChainAdapterError)

}

pub type Result<T, E = RuntimeAdapterError> = core::result::Result<T, E>;
