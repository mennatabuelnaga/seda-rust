//! WASI compatible WASM VM

pub mod adapters;
pub use adapters::*;

mod config;
pub use config::*;

mod context;
pub use context::*;

mod errors;
pub use errors::*;

pub(crate) mod imports;

mod promise;
pub(crate) use promise::*;

mod runtime;
pub use runtime::*;

// TODO move into cfg test
mod test_adapters;
pub use test_adapters::*;

#[cfg(test)]
#[path = ""]
pub mod test {
    use super::*;

    mod runtime_test;
    mod test_adapters;
}
