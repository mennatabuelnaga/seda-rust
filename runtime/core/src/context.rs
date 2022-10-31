use std::sync::Arc;

use parking_lot::Mutex;
use wasmer::{HostEnvInitError, Instance, LazyInit, Memory, WasmerEnv};

use super::PromiseQueue;

#[derive(Clone)]
pub struct VmContext {
    pub memory:                LazyInit<Memory>,
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
        current_promise_queue: Arc<Mutex<PromiseQueue>>,
        promise_queue: Arc<Mutex<PromiseQueue>>,
    ) -> VmContext {
        VmContext {
            memory: LazyInit::new(),
            current_promise_queue,
            promise_queue,
        }
    }
}
