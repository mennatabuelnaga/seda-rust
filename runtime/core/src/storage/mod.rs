mod in_memory_adapter;
pub use in_memory_adapter::*;

mod memory_adapter;
pub use memory_adapter::*;
pub(crate) use seda_runtime_sdk::{Bytes, FromBytes, ToBytes};

#[cfg(test)]
#[path = ""]
pub mod test {
    use super::*;

    mod in_memory_adapter_test;
}
