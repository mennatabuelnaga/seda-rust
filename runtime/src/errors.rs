use std::{array::TryFromSliceError, str::Utf8Error};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Failed to convert bytes to String: `{0}`")]
    StringBytesConversion(#[from] Utf8Error),
    #[error("Failed to convert bytes to Number: `{0}`")]
    NumBytesConversion(#[from] TryFromSliceError),
}

pub type Result<T, E = RuntimeError> = core::result::Result<T, E>;
