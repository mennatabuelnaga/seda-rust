use std::{array::TryFromSliceError, str::Utf8Error};

use error_stack::Report;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("{0:?}")]
    StringBytesConversion(Report<Utf8Error>),
    #[error("{0}")]
    NumBytesConversion(Report<TryFromSliceError>),
}

impl From<Report<Utf8Error>> for RuntimeError {
    fn from(r: Report<Utf8Error>) -> Self {
        Self::StringBytesConversion(r)
    }
}

impl From<Report<TryFromSliceError>> for RuntimeError {
    fn from(r: Report<TryFromSliceError>) -> Self {
        Self::NumBytesConversion(r)
    }
}

pub type Result<T, E = RuntimeError> = core::result::Result<T, E>;
