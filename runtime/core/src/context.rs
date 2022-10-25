use std::sync::{Arc, Mutex};

use wasmer::{HostEnvInitError, Instance, LazyInit, Memory, WasmerEnv};

use super::{AdapterTypes, Adapters, PromiseQueue, PromiseStatus};

#[derive(Clone)]
pub struct VmContext<Types>
where
    Types: AdapterTypes,
{
    pub memory:                LazyInit<Memory>,
    pub promise_queue:         Arc<Mutex<PromiseQueue>>,
    pub prev_promise_statuses: Arc<Mutex<Vec<PromiseStatus>>>,
    pub adapters:              Arc<Mutex<Adapters<Types>>>,
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
    pub fn create_vm_context(
        adapters: Arc<Mutex<Adapters<Types>>>,
        prev_promise_statuses: Arc<Mutex<Vec<PromiseStatus>>>,
        promise_queue: Arc<Mutex<PromiseQueue>>,
    ) -> VmContext<Types> {
        VmContext {
            memory: LazyInit::new(),
            prev_promise_statuses,
            promise_queue,
            adapters,
        }
    }
}
