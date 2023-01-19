use std::{io::Read, sync::Arc};

use parking_lot::Mutex;
use seda_config::{ChainConfigs, NodeConfig};
use seda_runtime_sdk::{CallSelfAction, Promise, PromiseAction, PromiseStatus};
use serde::{Deserialize, Serialize};
use tracing::info;
use wasmer::{Instance, Module, Store};
use wasmer_wasi::{Pipe, WasiState};

use super::{imports::create_wasm_imports, PromiseQueue, Result, VmConfig, VmContext};
use crate::{HostAdapter, InMemory, RuntimeError};

#[derive(Clone)]
pub struct Runtime<HA: HostAdapter> {
    wasm_module:      Option<Module>,
    limited:          bool,
    pub host_adapter: HA,
    pub node_config:  NodeConfig,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct VmResult {
    pub output:    Vec<String>,
    pub result:    Vec<u8>,
    pub exit_code: u8,
}

#[async_trait::async_trait]
pub trait RunnableRuntime {
    async fn new(node_config: NodeConfig, chains_config: ChainConfigs, limited: bool) -> Result<Self>
    where
        Self: Sized;
    fn init(&mut self, wasm_binary: Vec<u8>) -> Result<()>;

    async fn execute_promise_queue(
        &self,
        wasm_module: &Module,
        memory_adapter: Arc<Mutex<InMemory>>,
        promise_queue: PromiseQueue,
        output: &mut Vec<String>,

        // Getting the results of all the promise queues
        // Used to get the result of the last execution (for JSON RPC)
        // Can also be used to debug the queue
        promise_queue_trace: &mut Vec<PromiseQueue>,
    ) -> Result<u8>;

    async fn start_runtime(&self, config: VmConfig, memory_adapter: Arc<Mutex<InMemory>>) -> Result<VmResult>;
}

#[async_trait::async_trait]
impl<HA: HostAdapter> RunnableRuntime for Runtime<HA> {
    async fn new(node_config: NodeConfig, chains_config: ChainConfigs, limited: bool) -> Result<Self> {
        Ok(Self {
            wasm_module: None,
            limited,
            host_adapter: HA::new(chains_config)
                .await
                .map_err(|e| RuntimeError::NodeError(e.to_string()))?,
            node_config,
        })
    }

    /// Initializes the runtime, this speeds up VM execution by caching WASM
    /// binary parsing
    fn init(&mut self, wasm_binary: Vec<u8>) -> Result<()> {
        let wasm_store = Store::default();
        let wasm_module = Module::new(&wasm_store, wasm_binary)?;

        self.wasm_module = Some(wasm_module);

        Ok(())
    }

    async fn execute_promise_queue(
        &self,
        wasm_module: &Module,
        memory_adapter: Arc<Mutex<InMemory>>,
        promise_queue: PromiseQueue,
        output: &mut Vec<String>,
        promise_queue_trace: &mut Vec<PromiseQueue>,
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
                    action if self.limited && action.is_limited_action() => {
                        promise_queue_mut.queue[index].status = PromiseStatus::Rejected(
                            format!("Method `{}` not allowed in limited runtime", action).into_bytes(),
                        )
                    }
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
                            if !stdout_buffer.is_empty() {
                                output.push(stdout_buffer);
                            }
                        }

                        let mut wasi_state = wasi_env.state();
                        let wasi_stderr = wasi_state.fs.stderr_mut()?.as_mut().unwrap();
                        let mut stderr_buffer = String::new();
                        wasi_stderr.read_to_string(&mut stderr_buffer)?;
                        if !stderr_buffer.is_empty() {
                            output.push(stderr_buffer);
                        }

                        // Unwrap the error here after capturing the output
                        // otherwise the output would get lost
                        if let Err(err) = runtime_result {
                            info!("WASM Error output: {:?}", &output);
                            return Err(RuntimeError::ExecutionError(err));
                        }

                        let execution_result = vm_context.result.lock();
                        next_promise_queue = next_queue.lock().clone();
                        promise_queue_mut.queue[index].status = PromiseStatus::Fulfilled(execution_result.clone());
                    }

                    // Just an example, delete this later
                    PromiseAction::DatabaseSet(db_action) => {
                        self.host_adapter
                            .db_set(&db_action.key, &String::from_utf8(db_action.value.clone())?)
                            .await
                            .map_err(|e| RuntimeError::NodeError(e.to_string()))?;

                        promise_queue_mut.queue[index].status = PromiseStatus::Fulfilled(vec![]);
                    }

                    PromiseAction::DatabaseGet(db_action) => {
                        let result = self
                            .host_adapter
                            .db_get(&db_action.key)
                            .await
                            .map_err(|e| RuntimeError::NodeError(e.to_string()))?;

                        match result {
                            Some(r) => {
                                promise_queue_mut.queue[index].status =
                                    PromiseStatus::Fulfilled(r.to_string().into_bytes())
                            }
                            None => promise_queue_mut.queue[index].status = PromiseStatus::Rejected(vec![]),
                        }
                    }

                    PromiseAction::Http(http_action) => {
                        let resp = self
                            .host_adapter
                            .http_fetch(&http_action.url)
                            .await
                            .map_err(|e| RuntimeError::NodeError(e.to_string()))?;

                        promise_queue_mut.queue[index].status = PromiseStatus::Fulfilled(resp.into_bytes());
                    }
                    PromiseAction::ChainView(chain_view_action) => {
                        let resp = self
                            .host_adapter
                            .chain_view(
                                chain_view_action.chain,
                                &chain_view_action.contract_id,
                                &chain_view_action.method_name,
                                chain_view_action.args.clone(),
                            )
                            .await
                            .map_err(|e| RuntimeError::NodeError(e.to_string()))?;

                        promise_queue_mut.queue[index].status = PromiseStatus::Fulfilled(resp);
                    }
                    PromiseAction::ChainCall(chain_call_action) => {
                        let resp = self
                            .host_adapter
                            .chain_call(
                                chain_call_action.chain,
                                &chain_call_action.contract_id,
                                &chain_call_action.method_name,
                                chain_call_action.args.clone(),
                                chain_call_action.deposit.parse::<u128>()?,
                                self.node_config.clone(),
                            )
                            .await
                            .map_err(|e| RuntimeError::NodeError(e.to_string()))?;

                        promise_queue_mut.queue[index].status = PromiseStatus::Fulfilled(resp);
                    }
                    PromiseAction::TriggerEvent(trigger_event_action) => {
                        self.host_adapter
                            .trigger_event(trigger_event_action.event.clone())
                            .await
                            .map_err(|e| RuntimeError::NodeError(e.to_string()))?;

                        promise_queue_mut.queue[index].status = PromiseStatus::Fulfilled(vec![]);
                    }
                }
            }
        }

        promise_queue_trace.push(promise_queue_mut.clone());

        let res = self.execute_promise_queue(
            wasm_module,
            memory_adapter.clone(),
            next_promise_queue,
            output,
            promise_queue_trace,
        );

        res.await
    }

    async fn start_runtime(&self, config: VmConfig, memory_adapter: Arc<Mutex<InMemory>>) -> Result<VmResult> {
        let function_name = config.clone().start_func.unwrap_or_else(|| "_start".to_string());
        let wasm_module = self.wasm_module.as_ref().unwrap();

        let mut promise_queue_trace: Vec<PromiseQueue> = Vec::new();
        let mut promise_queue = PromiseQueue::new();

        promise_queue.add_promise(Promise {
            action: PromiseAction::CallSelf(CallSelfAction {
                function_name,
                args: config.args,
            }),
            status: PromiseStatus::Unfulfilled,
        });

        let mut output: Vec<String> = vec![];

        let exit_code = self
            .execute_promise_queue(
                wasm_module,
                memory_adapter,
                promise_queue,
                &mut output,
                &mut promise_queue_trace,
            )
            .await?;

        // There is always 1 queue with 1 promise in the trace (due this func addinging
        // the entrypoint)
        let last_queue = promise_queue_trace.last().ok_or("Failed to get last promise queue")?;
        let last_promise_status = last_queue
            .queue
            .last()
            .ok_or("Failed to get last promise in promise queue")?
            .status
            .clone();

        let result_data = match last_promise_status {
            PromiseStatus::Fulfilled(data) => data,
            PromiseStatus::Rejected(data) => data,
            _ => vec![],
        };

        Ok(VmResult {
            output,
            exit_code,
            result: result_data,
        })
    }
}
