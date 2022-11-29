use seda_runtime_adapters::RuntimeAdapterError;
use thiserror::Error;
use wasmer::{CompileError, ExportError, InstantiationError};
use wasmer_wasi::{FsError, WasiError, WasiStateCreationError};

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error(transparent)]
    WasmCompileError(#[from] CompileError),

    #[error(transparent)]
    WasmInstantiationError(Box<InstantiationError>),

    #[error(transparent)]
    WasiError(#[from] WasiError),
    #[error(transparent)]
    WasiStateCreationError(#[from] WasiStateCreationError),

    #[error(transparent)]
    FunctionNotFound(#[from] ExportError),

    #[error("Error while running: {0}")]
    ExecutionError(#[from] wasmer::RuntimeError),

    #[error("VM Host Error: {0}")]
    VmHostError(String),

    #[error(transparent)]
    WasiFsError(#[from] FsError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    RuntimeAdapterError(#[from] RuntimeAdapterError),
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

pub type Result<T, E = RuntimeError> = core::result::Result<T, E>;
