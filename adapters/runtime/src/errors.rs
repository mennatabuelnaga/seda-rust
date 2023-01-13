use std::{array::TryFromSliceError, num::ParseIntError, str::Utf8Error};

use actix::MailboxError;
use seda_chain_adapters::ChainAdapterError;
use thiserror::Error;
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

    #[error("Mailbox Error Error: {0}")]
    MailboxError(#[from] MailboxError),

    #[error("Parse Integer Error: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("Chain Adapter Error: {0}")]
    ChainAdapterError(#[from] ChainAdapterError),
}

pub type Result<T, E = RuntimeAdapterError> = core::result::Result<T, E>;
