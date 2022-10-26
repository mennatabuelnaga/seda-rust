use std::sync::{Arc, Mutex};

use wasmer::{HostEnvInitError, Instance, LazyInit, Memory, WasmerEnv};

use super::{PromiseQueue, PromiseStatus};

#[derive(Clone)]
pub struct VmContext {
    pub memory:                LazyInit<Memory>,
    pub promise_queue:         Arc<Mutex<PromiseQueue>>,
    pub prev_promise_statuses: Arc<Mutex<Vec<PromiseStatus>>>,
}

impl WasmerEnv for VmContext {
    fn init_with_instance(&mut self, instance: &Instance) -> Result<(), HostEnvInitError> {
        let memory: Memory = instance.exports.get_with_generics_weak("memory").unwrap();
        self.memory.initialize(memory);

        Ok(())
    }
}

impl VmContext {
    pub fn create_vm_context(
        prev_promise_statuses: Arc<Mutex<Vec<PromiseStatus>>>,
        promise_queue: Arc<Mutex<PromiseQueue>>,
    ) -> VmContext {
        VmContext {
            memory: LazyInit::new(),
            prev_promise_statuses,
            promise_queue,
        }
    }
}
