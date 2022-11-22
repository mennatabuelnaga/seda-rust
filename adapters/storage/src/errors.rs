use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageAdapterError {
    #[error("Rusqlite Error: {0}")]
    RuqliteError(#[from] rusqlite::Error),
}

pub type Result<T, E = StorageAdapterError> = core::result::Result<T, E>;
