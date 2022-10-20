use std::sync::{Arc, Mutex};

use mockall::automock;
use wasmer::{HostEnvInitError, Instance, LazyInit, Memory, WasmerEnv};

use crate::{
    adapters::Adapters,
    promise::{Promise, PromiseQueue, PromiseQueueBP},
};

#[derive(Clone)]
pub struct VmContext {
    pub memory:   LazyInit<Memory>,
    pub promises: Arc<Mutex<PromiseQueue>>,
    pub adapters: Arc<Mutex<Adapters>>,
}

impl WasmerEnv for VmContext {
    fn init_with_instance(&mut self, instance: &Instance) -> Result<(), HostEnvInitError> {
        let memory: Memory = instance.exports.get_with_generics_weak("memory").unwrap();
        self.memory.initialize(memory);

        Ok(())
    }
}

pub fn create_vm_context(adapters: Arc<Mutex<Adapters>>) -> VmContext {
    let promise_queue = PromiseQueue::new();

    VmContext {
        memory: LazyInit::new(),
        promises: Arc::new(Mutex::new(promise_queue)),
        adapters,
    }
}
