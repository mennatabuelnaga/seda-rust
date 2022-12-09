//! WASI compatible WASM VM

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

#[cfg(test)]
#[path = ""]
pub mod test {
    #[cfg(feature = "near")]
    mod near_runtime_test;

    #[cfg(not(feature = "near"))]
    mod runtime_test;
}
