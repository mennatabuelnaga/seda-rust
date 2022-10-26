use std::sync::{Arc, Mutex};

use wasmer::{Instance, Module, Store};
use wasmer_wasi::WasiState;

use super::Result;
use crate::{
    adapters::{AdapterTypes, Adapters, DatabaseAdapter},
    config::VmConfig,
    context::VmContext,
    imports::create_wasm_imports,
    promise::{
        promise_actions::{CallSelfAction, PromiseAction},
        Promise,
        PromiseQueue,
        PromiseStatus,
    },
};

pub struct VmResult {}

fn execute_promise_queue<Types: AdapterTypes>(
    wasm_module: Module,
    promise_queue: PromiseQueue,
    adapters: Arc<Mutex<Adapters<Types>>>,
) -> Result<VmResult> {
    // This queue will be used in the current execution
    // We should not use the same promise_queue otherwise getting results back would
    // be hard to do due the indexes of results (will be hard to refactor)
    let next_promise_queue = Arc::new(Mutex::new(PromiseQueue::new()));
    let mut statuses = vec![PromiseStatus::Unfulfilled; promise_queue.queue.len()];

    for (index, promise) in promise_queue.queue.iter().enumerate() {
        statuses[index] = PromiseStatus::Pending;

        match &promise.action {
            PromiseAction::CallSelf(call_action) => {
                let wasm_store = Store::default();
                let mut wasi_env = WasiState::new(&call_action.function_name)
                    .args(call_action.args.clone())
                    .finalize()?;

                let promise_statuses = Arc::new(Mutex::new(statuses.clone()));

                let vm_context =
                    VmContext::create_vm_context(adapters.clone(), promise_statuses, next_promise_queue.clone());
                let imports = create_wasm_imports(&wasm_store, vm_context.clone(), &mut wasi_env, &wasm_module);
                let wasmer_instance = Instance::new(&wasm_module, &imports).unwrap();

                let main_func = wasmer_instance.exports.get_function(&call_action.function_name)?;

                main_func.call(&[])?;
                statuses[index] = PromiseStatus::Fulfilled(vec![]);
            }

            // Just an example, delete this later
            PromiseAction::DatabaseSet(db_action) => {
                let mut adapter_ref = adapters.lock().unwrap();

                adapter_ref
                    .database
                    .set(&db_action.key, &String::from_utf8(db_action.value.clone()).unwrap());

                statuses[index] = PromiseStatus::Fulfilled(vec![]);
            }

            PromiseAction::DatabaseGet(db_action) => {
                let adapter_ref = adapters.lock().unwrap();
                let result = adapter_ref.database.get(&db_action.key).unwrap();

                statuses[index] = PromiseStatus::Fulfilled(result.to_string().into_bytes());
            }
        }
    }

    let next_promise_queue_ref = next_promise_queue.lock();
    let deref_next_promise_queue = next_promise_queue_ref.unwrap().to_owned();

    if !deref_next_promise_queue.queue.is_empty() {
        return execute_promise_queue(wasm_module, deref_next_promise_queue, adapters);
    }

    Result::Ok(VmResult {})
}

pub fn start_runtime<Types: AdapterTypes>(config: VmConfig, adapters: Arc<Mutex<Adapters<Types>>>) -> Result<VmResult> {
    let wasm_store = Store::default();
    let function_name = config.clone().start_func.unwrap_or_else(|| "_start".to_string());
    let wasm_module = Module::new(&wasm_store, &config.wasm_binary)?;

    let mut promise_queue = PromiseQueue::new();

    promise_queue.add_promise(Promise {
        action: PromiseAction::CallSelf(CallSelfAction {
            function_name,
            args: config.args,
        }),
        status: PromiseStatus::Unfulfilled,
    });

    execute_promise_queue(wasm_module, promise_queue, adapters)
}
