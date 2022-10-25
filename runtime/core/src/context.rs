use std::sync::{Arc, Mutex};

use wasmer::{HostEnvInitError, Instance, LazyInit, Memory, WasmerEnv};

use super::{AdapterTypes, Adapters, PromiseQueue, PromiseQueueBP};

#[derive(Clone)]
pub struct VmContext<Types>
where
    Types: AdapterTypes,
{
    pub memory:   LazyInit<Memory>,
    pub promises: Arc<Mutex<PromiseQueue>>,
    pub adapters: Arc<Mutex<Adapters<Types>>>,
}

impl<Types> WasmerEnv for VmContext<Types>
where
    Types: AdapterTypes,
{
    fn init_with_instance(&mut self, instance: &Instance) -> Result<(), HostEnvInitError> {
        let memory: Memory = instance.exports.get_with_generics_weak("memory").unwrap();
        self.memory.initialize(memory);

        Ok(())
    }
}

impl<Types> VmContext<Types>
where
    Types: AdapterTypes,
{
    pub fn create_vm_context(adapters: Arc<Mutex<Adapters<Types>>>) -> VmContext<Types> {
        let promise_queue = PromiseQueue::new();

        VmContext {
            memory: LazyInit::new(),
            promises: Arc::new(Mutex::new(promise_queue)),
            adapters,
        }
    }
}
