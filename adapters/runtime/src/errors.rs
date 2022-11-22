use std::{array::TryFromSliceError, str::Utf8Error};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeAdapterError {
    #[error("{0:?}")]
    StringBytesConversion(#[from] Utf8Error),
    #[error("{0}")]
    NumBytesConversion(#[from] TryFromSliceError),
}

pub type Result<T, E = RuntimeAdapterError> = core::result::Result<T, E>;
