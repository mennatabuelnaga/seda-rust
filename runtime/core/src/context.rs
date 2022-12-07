use std::sync::Arc;

use parking_lot::Mutex;
use seda_runtime_adapters::InMemory;
use wasmer::{AsStoreRef, FunctionEnv, Memory, MemoryView, Store};

use super::PromiseQueue;

#[derive(Clone)]
pub struct VmContext {
    pub memory:                Option<Memory>,
    pub memory_adapter:        Arc<Mutex<InMemory>>,
    pub promise_queue:         Arc<Mutex<PromiseQueue>>,
    pub current_promise_queue: Arc<Mutex<PromiseQueue>>,
}

impl VmContext {
    pub fn create_vm_context(
        store: &mut Store,
        memory_adapter: Arc<Mutex<InMemory>>,
        current_promise_queue: Arc<Mutex<PromiseQueue>>,
        promise_queue: Arc<Mutex<PromiseQueue>>,
    ) -> FunctionEnv<VmContext> {
        FunctionEnv::new(
            store,
            VmContext {
                memory_adapter,
                memory: None,
                current_promise_queue,
                promise_queue,
            },
        )
    }

    /// Providers safe access to the memory
    /// (it must be initialized before it can be used)
    pub fn memory_view<'a>(&'a self, store: &'a impl AsStoreRef) -> MemoryView<'a> {
        self.memory().view(store)
    }

    /// Get memory, that needs to have been set fist
    pub fn memory(&self) -> &Memory {
        self.memory.as_ref().unwrap()
    }
}
