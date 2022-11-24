use std::{future::Future, io::Read, process::Output, sync::Arc};

use parking_lot::Mutex;
use seda_runtime_sdk::{CallSelfAction, Promise, PromiseAction, PromiseStatus};
use serde::{Deserialize, Serialize};
use wasmer::{Instance, Module, Store};
use wasmer_wasi::{Pipe, WasiState};

use super::{imports::create_wasm_imports, HostAdapterTypes, HostAdapters, PromiseQueue, Result, VmConfig, VmContext};
use crate::{promise::promise_queue, InMemory};

#[derive(Clone)]
pub struct Runtime {
    wasm_module: Option<Module>,
}

#[async_trait::async_trait]
pub trait ExampleTrait: Send {
    async fn my_callback();
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct VmResult {
    pub output:    Vec<String>,
    pub exit_code: u8,
}

#[async_trait::async_trait]
pub trait RunnableRuntime {
    fn new() -> Self;
    fn init(&mut self, wasm_binary: Vec<u8>) -> Result<()>;

    async fn execute_promise_queue<T: HostAdapterTypes + Default, Ex: ExampleTrait>(
        &self,
        wasm_module: &Module,
        memory_adapter: Arc<Mutex<InMemory>>,
        promise_queue: PromiseQueue,
        host_adapters: HostAdapters<T>,
        output: &mut Vec<String>,
        // callback: F,
    ) -> Result<u8>;

    async fn start_runtime<T: HostAdapterTypes + Default, Ex: ExampleTrait>(
        &self,
        config: VmConfig,
        memory_adapter: Arc<Mutex<InMemory>>,
        host_adapters: HostAdapters<T>,
        // callback: F,
    ) -> Result<VmResult>;
}

#[async_trait::async_trait]
impl RunnableRuntime for Runtime {
    fn new() -> Self {
        Self { wasm_module: None }
    }

    /// Initializes the runtime, this speeds up VM execution by caching WASM
    /// binary parsing
    fn init(&mut self, wasm_binary: Vec<u8>) -> Result<()> {
        let wasm_store = Store::default();
        let wasm_module = Module::new(&wasm_store, wasm_binary)?;

        self.wasm_module = Some(wasm_module);

        Ok(())
    }

    async fn execute_promise_queue<T: HostAdapterTypes + Default, Ex: ExampleTrait>(
        &self,
        wasm_module: &Module,
        memory_adapter: Arc<Mutex<InMemory>>,
        promise_queue: PromiseQueue,
        host_adapters: HostAdapters<T>,
        output: &mut Vec<String>,
        // callback: F,
    ) -> Result<u8> {
        let mut next_promise_queue = PromiseQueue::new();
        let mut promise_queue_mut = promise_queue.clone();

        {
            // This queue will be used in the current execution
            // We should not use the same promise_queue otherwise getting results back would
            // be hard to do due the indexes of results (will be hard to refactor)

            if promise_queue.queue.is_empty() {
                return Ok(0);
            }

            for index in 0..promise_queue.queue.len() {
                promise_queue_mut.queue[index].status = PromiseStatus::Pending;

                match &promise_queue.queue[index].action {
                    PromiseAction::CallSelf(call_action) => {
                        let wasm_store = Store::default();
                        let stdout_pipe = Pipe::new();
                        let stderr_pipe = Pipe::new();

                        let mut wasi_env = WasiState::new(&call_action.function_name)
                            .args(call_action.args.clone())
                            .stdout(Box::new(stdout_pipe))
                            .stderr(Box::new(stderr_pipe))
                            .finalize()?;

                        let current_promise_queue = Arc::new(Mutex::new(promise_queue_mut.clone()));
                        let next_queue = Arc::new(Mutex::new(PromiseQueue::new()));

                        let vm_context = VmContext::create_vm_context(
                            memory_adapter.clone(),
                            current_promise_queue,
                            next_queue.clone(),
                        );

                        let imports = create_wasm_imports(&wasm_store, vm_context.clone(), &mut wasi_env, wasm_module)?;
                        let wasmer_instance = Instance::new(wasm_module, &imports)?;

                        let main_func = wasmer_instance.exports.get_function(&call_action.function_name)?;

                        let runtime_result = main_func.call(&[]);

                        {
                            // We need to use the wasi_state twice (which is not clonable) so this
                            // puts into scope the wasi_state so the MutexGuard gets unlocked after
                            let mut wasi_state = wasi_env.state();
                            let wasi_stdout = wasi_state.fs.stdout_mut()?.as_mut().unwrap();
                            let mut stdout_buffer = String::new();
                            wasi_stdout.read_to_string(&mut stdout_buffer)?;
                            output.push(stdout_buffer);
                        }

                        let mut wasi_state = wasi_env.state();
                        let wasi_stderr = wasi_state.fs.stderr_mut()?.as_mut().unwrap();
                        let mut stderr_buffer = String::new();
                        wasi_stderr.read_to_string(&mut stderr_buffer)?;
                        output.push(stderr_buffer);

                        // Unwrap the error here after capturing the output
                        // otherwise the output would get lost
                        runtime_result?;
                        next_promise_queue = next_queue.lock().clone();
                        promise_queue_mut.queue[index].status = PromiseStatus::Fulfilled(vec![]);
                    }

                    // Just an example, delete this later
                    PromiseAction::DatabaseSet(db_action) => {
                        host_adapters
                            .db_set(&db_action.key, &String::from_utf8(db_action.value.clone()).unwrap())
                            .unwrap();

                        // let _r = callback().await;
                        let r = Ex::my_callback().await;

                        promise_queue_mut.queue[index].status = PromiseStatus::Fulfilled(vec![]);
                    }

                    PromiseAction::DatabaseGet(db_action) => {
                        let result = host_adapters.db_get(&db_action.key).unwrap().unwrap();

                        promise_queue_mut.queue[index].status =
                            PromiseStatus::Fulfilled(result.to_string().into_bytes());
                    }

                    PromiseAction::Http(http_action) => {
                        let resp = host_adapters.http_fetch(&http_action.url).unwrap();
                        let r = Ex::my_callback().await;

                        promise_queue_mut.queue[index].status = PromiseStatus::Fulfilled(resp.into_bytes());
                    }
                }
            }
        }

        let res = self.execute_promise_queue::<T, Ex>(
            wasm_module,
            memory_adapter.clone(),
            next_promise_queue,
            host_adapters,
            output,
            // callback,
        );

        res.await
    }

    async fn start_runtime<T: HostAdapterTypes + Default, Ex: ExampleTrait>(
        &self,
        config: VmConfig,
        memory_adapter: Arc<Mutex<InMemory>>,
        host_adapters: HostAdapters<T>,
        // callback: F,
    ) -> Result<VmResult> {
        // callback().await;
        let function_name = config.clone().start_func.unwrap_or_else(|| "_start".to_string());
        let wasm_module = self.wasm_module.as_ref().unwrap();

        let mut promise_queue = PromiseQueue::new();

        promise_queue.add_promise(Promise {
            action: PromiseAction::CallSelf(CallSelfAction {
                function_name,
                args: config.args,
            }),
            status: PromiseStatus::Unfulfilled,
        });

        let mut output: Vec<String> = vec![];

        let result = self
            .execute_promise_queue::<T, Ex>(
                wasm_module,
                memory_adapter,
                promise_queue,
                host_adapters,
                &mut output,
                // callback,
            )
            .await;

        Ok(VmResult {
            output,
            exit_code: result.unwrap_or(1),
        })
    }
}
