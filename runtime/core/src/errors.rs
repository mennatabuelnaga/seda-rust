use seda_http_adapters::HttpAdapterError;
use seda_runtime_adapters::RuntimeAdapterError;
use seda_storage_adapters::StorageAdapterError;
use thiserror::Error;
use wasmer::{CompileError, ExportError, InstantiationError};
use wasmer_wasi::{FsError, WasiError, WasiStateCreationError};

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("{0:?}")]
    StringBytesConversion(Report<Utf8Error>),

    #[error("{0}")]
    NumBytesConversion(Report<TryFromSliceError>),

    #[error("{0}")]
    WasmCompileError(#[from] CompileError),

    #[error("{0}")]
    WasmInstantiationError(Box<InstantiationError>),

    #[error("{0}")]
    WasiError(#[from] WasiError),
    #[error("{0}")]
    WasiStateCreationError(#[from] WasiStateCreationError),

    #[error("{0}")]
    FunctionNotFound(#[from] ExportError),

    #[error("Error while running: {0}")]
    ExecutionError(#[from] wasmer::RuntimeError),

    #[error("VM Host Error: {0}")]
    VmHostError(String),

    #[error("Storage Adapter Error: {0}")]
    StorageAdapterError(#[from] StorageAdapterError),
    #[error("Http Adapter Error: {0}")]
    HttpAdapterError(#[from] HttpAdapterError),

    #[error("{0}")]
    WasiFsError(FsError),

    #[error("{0}")]
    IoError(std::io::Error),

    #[error("{0}")]
    FromUtf8Error(std::string::FromUtf8Error),
}

impl From<InstantiationError> for RuntimeError {
    fn from(r: InstantiationError) -> Self {
        Self::WasmInstantiationError(Box::new(r))
    }
}

impl From<serde_json::Error> for RuntimeError {
    fn from(s: serde_json::Error) -> Self {
        Self::VmHostError(s.to_string())
    }
}

impl From<String> for RuntimeError {
    fn from(s: String) -> Self {
        Self::VmHostError(s)
    }
}

impl From<&str> for RuntimeError {
    fn from(s: &str) -> Self {
        Self::VmHostError(s.into())
    }
}

impl From<FsError> for RuntimeError {
    fn from(e: FsError) -> Self {
        Self::WasiFsError(e)
    }
}

impl From<std::io::Error> for RuntimeError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<std::string::FromUtf8Error> for RuntimeError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8Error(e)
    }
}

pub type Result<T, E = RuntimeError> = core::result::Result<T, E>;
