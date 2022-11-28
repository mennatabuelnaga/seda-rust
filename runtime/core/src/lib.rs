//! WASI compatible WASM VM

pub mod adapters;
pub use adapters::*;

mod bytes;
pub use bytes::*;

mod config;
pub use config::*;

mod context;
pub use context::*;

mod errors;
pub use errors::*;

pub(crate) mod imports;

mod in_memory_adapter;
pub use in_memory_adapter::*;

mod memory_adapter;
pub use memory_adapter::*;

mod promise;
pub(crate) use promise::*;

mod runtime;
pub use runtime::*;

#[cfg(test)]
#[path = ""]
pub mod test {
    use super::*;

    mod in_memory_adapter_test;
    mod runtime_test;
}
