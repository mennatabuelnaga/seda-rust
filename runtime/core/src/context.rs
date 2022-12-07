use std::sync::Arc;

use parking_lot::Mutex;
use seda_runtime_adapters::InMemory;
use wasmer::{HostEnvInitError, Instance, LazyInit, Memory, WasmerEnv};

use super::PromiseQueue;

#[derive(Clone)]
pub struct VmContext {
    pub result:                Arc<Mutex<Vec<u8>>>,
    pub memory:                LazyInit<Memory>,
    pub memory_adapter:        Arc<Mutex<InMemory>>,
    pub promise_queue:         Arc<Mutex<PromiseQueue>>,
    pub current_promise_queue: Arc<Mutex<PromiseQueue>>,
}

impl WasmerEnv for VmContext {
    fn init_with_instance(&mut self, instance: &Instance) -> Result<(), HostEnvInitError> {
        let memory: Memory = instance.exports.get_with_generics_weak("memory")?;
        self.memory.initialize(memory);

        Ok(())
    }
}

impl VmContext {
    pub fn create_vm_context(
        memory_adapter: Arc<Mutex<InMemory>>,
        current_promise_queue: Arc<Mutex<PromiseQueue>>,
        promise_queue: Arc<Mutex<PromiseQueue>>,
    ) -> VmContext {
        VmContext {
            result: Arc::new(Mutex::new(Vec::new())),
            memory_adapter,
            memory: LazyInit::new(),
            current_promise_queue,
            promise_queue,
        }
    }
}
