use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpAdapterError {
    #[error("Rusqlite Error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

pub type Result<T, E = HttpAdapterError> = core::result::Result<T, E>;
