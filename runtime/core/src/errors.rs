use std::{array::TryFromSliceError, str::Utf8Error};

use error_stack::Report;
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
    WasmCompileError(Report<CompileError>),

    #[error("{0}")]
    WasmInstantiationError(Report<InstantiationError>),

    #[error("{0}")]
    WasiError(Report<WasiError>),

    #[error("{0}")]
    WasiStateCreationError(Report<WasiStateCreationError>),

    #[error("{0}")]
    FunctionNotFound(Report<ExportError>),

    #[error("Error while running: {0}")]
    ExecutionError(Report<wasmer::RuntimeError>),

    #[error("VM Host Error: {0}")]
    VmHostError(String),

    #[error("{0}")]
    WasiFsError(FsError),

    #[error("{0}")]
    IoError(std::io::Error),
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

impl From<CompileError> for RuntimeError {
    fn from(r: CompileError) -> Self {
        Self::WasmCompileError(r.into())
    }
}

impl From<InstantiationError> for RuntimeError {
    fn from(r: InstantiationError) -> Self {
        Self::WasmInstantiationError(r.into())
    }
}

impl From<WasiError> for RuntimeError {
    fn from(r: WasiError) -> Self {
        Self::WasiError(r.into())
    }
}

impl From<WasiStateCreationError> for RuntimeError {
    fn from(r: WasiStateCreationError) -> Self {
        Self::WasiStateCreationError(r.into())
    }
}

impl From<ExportError> for RuntimeError {
    fn from(r: ExportError) -> Self {
        Self::FunctionNotFound(r.into())
    }
}

impl From<wasmer::RuntimeError> for RuntimeError {
    fn from(r: wasmer::RuntimeError) -> Self {
        Self::ExecutionError(r.into())
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
        Self::WasiFsError(e.into())
    }
}

impl From<std::io::Error> for RuntimeError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

pub type Result<T, E = RuntimeError> = core::result::Result<T, E>;
