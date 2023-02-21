use thiserror::Error;

#[derive(Debug, Error)]
pub enum SDKError {
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error("{0:?}")]
    StringBytesConversion(#[from] std::str::Utf8Error),

    #[error(transparent)]
    NumBytesConversion(#[from] std::array::TryFromSliceError),
}

pub type Result<T, E = SDKError> = core::result::Result<T, E>;
