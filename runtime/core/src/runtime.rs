use std::{io::Read, sync::Arc};

use parking_lot::Mutex;
use seda_runtime_sdk::{CallSelfAction, Promise, PromiseAction, PromiseStatus};
use serde::{Deserialize, Serialize};
use wasmer::{Instance, Module, Store};
use wasmer_wasi::{Pipe, Stdout, WasiState};

use super::{imports::create_wasm_imports, HostAdapterTypes, HostAdapters, PromiseQueue, Result, VmConfig, VmContext};
use crate::InMemory;

#[derive(Clone, Default)]
pub struct Runtime {}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct VmResult {
    pub output: Vec<String>,
}

#[async_trait::async_trait]
pub trait RunnableRuntime {
    async fn execute_promise_queue<T: HostAdapterTypes + Default>(
        &self,
        wasm_module: Module,
        memory_adapter: Arc<Mutex<InMemory>>,
        promise_queue: Arc<Mutex<PromiseQueue>>,
        host_adapters: HostAdapters<T>,
        mut output: Vec<String>,
    ) -> Result<VmResult>;

    async fn start_runtime<T: HostAdapterTypes + Default>(
        &self,
        config: VmConfig,
        memory_adapter: Arc<Mutex<InMemory>>,
        host_adapters: HostAdapters<T>,
    ) -> Result<VmResult>;
}

#[async_trait::async_trait]
impl RunnableRuntime for Runtime {
    async fn execute_promise_queue<T: HostAdapterTypes + Default>(
        &self,
        wasm_module: Module,
        memory_adapter: Arc<Mutex<InMemory>>,
        promise_queue: Arc<Mutex<PromiseQueue>>,
        host_adapters: HostAdapters<T>,
        mut output: Vec<String>,
    ) -> Result<VmResult> {
        let next_promise_queue = Arc::new(Mutex::new(PromiseQueue::new()));
        {
            // This queue will be used in the current execution
            // We should not use the same promise_queue otherwise getting results back would
            // be hard to do due the indexes of results (will be hard to refactor)
            let mut promise_queue = promise_queue.lock();

            if promise_queue.queue.is_empty() {
                return Ok(VmResult { output });
            }

            for index in 0..promise_queue.queue.len() {
                promise_queue.queue[index].status = PromiseStatus::Pending;

                match &promise_queue.queue[index].action {
                    PromiseAction::CallSelf(call_action) => {
                        let wasm_store = Store::default();
                        let output_pipe = Pipe::new();

                        let mut wasi_env = WasiState::new(&call_action.function_name)
                            .args(call_action.args.clone())
                            .stdout(Box::new(output_pipe))
                            .finalize()?;

                        let current_promise_queue = Arc::new(Mutex::new(promise_queue.clone()));

                        let vm_context = VmContext::create_vm_context(
                            memory_adapter.clone(),
                            current_promise_queue,
                            next_promise_queue.clone(),
                        );

                        let imports =
                            create_wasm_imports(&wasm_store, vm_context.clone(), &mut wasi_env, &wasm_module)?;
                        let wasmer_instance = Instance::new(&wasm_module, &imports)?;

                        let main_func = wasmer_instance.exports.get_function(&call_action.function_name)?;

                        main_func.call(&[])?;

                        // let mut buf = String::new();
                        // .read_to_string(&mut buf)?;
                        let mut state = wasi_env.state();
                        let wasi_stdout = state.fs.stdout_mut().unwrap().as_mut().unwrap();
                        // Then we can read from it!
                        let mut buf = String::new();
                        wasi_stdout.read_to_string(&mut buf).unwrap();

                        output.push(buf);

                        promise_queue.queue[index].status = PromiseStatus::Fulfilled(vec![]);
                    }

                    // Just an example, delete this later
                    PromiseAction::DatabaseSet(db_action) => {
                        host_adapters.db_set(&db_action.key, &String::from_utf8(db_action.value.clone()).unwrap());

                        promise_queue.queue[index].status = PromiseStatus::Fulfilled(vec![]);
                    }

                    PromiseAction::DatabaseGet(db_action) => {
                        let result = host_adapters.db_get(&db_action.key).unwrap();

                        promise_queue.queue[index].status = PromiseStatus::Fulfilled(result.to_string().into_bytes());
                    }
                    PromiseAction::Http(http_action) => {
                        let resp = host_adapters.http_fetch(&http_action.url).unwrap();

                        promise_queue.queue[index].status = PromiseStatus::Fulfilled(resp.into_bytes());
                    }
                }
            }
        }

        let res = self.execute_promise_queue(
            wasm_module,
            memory_adapter.clone(),
            next_promise_queue,
            host_adapters,
            output,
        );
        res.await
    }

    async fn start_runtime<T: HostAdapterTypes + Default>(
        &self,
        config: VmConfig,
        memory_adapter: Arc<Mutex<InMemory>>,
        host_adapters: HostAdapters<T>,
    ) -> Result<VmResult> {
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

        self.execute_promise_queue(
            wasm_module,
            memory_adapter,
            Arc::new(Mutex::new(promise_queue)),
            host_adapters,
            vec![],
        )
        .await
    }
}
