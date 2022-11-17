use std::{fs, path::PathBuf, sync::Arc};

use actix::{prelude::*, Handler, Message};
use parking_lot::Mutex;
use seda_runtime::{HostAdapters, InMemory, RunnableRuntime, Runtime, VmConfig, VmResult};

use crate::{
    event_queue::{Event, EventData},
    test_adapters::TestAdapters,
};

#[derive(MessageResponse)]
pub struct RuntimeJobResult {
    pub vm_result: VmResult,
}

#[derive(Message)]
#[rtype(result = "RuntimeJobResult")]
pub struct RuntimeJob {
    pub event: Event,
}

pub struct RuntimeWorker;

impl Actor for RuntimeWorker {
    type Context = SyncContext<Self>;
}

impl Handler<RuntimeJob> for RuntimeWorker {
    type Result = RuntimeJobResult;

    fn handle(&mut self, msg: RuntimeJob, _ctx: &mut Self::Context) -> Self::Result {
        let host_adapters = HostAdapters::<TestAdapters>::default();
        // TODO: Replace the binary with the actual consensus binary
        let mut path_prefix = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path_prefix.push("./test_files/promise-wasm-bin.wasm");

        let runtime = Runtime {};
        let memory_adapter = Arc::new(Mutex::new(InMemory::default()));

        let args: Vec<String> = match msg.event.data {
            EventData::MainChainTick => vec![],
            EventData::CliCall(args) => args,
        };

        let vm_config = VmConfig {
            args,
            program_name: "test".to_string(),
            debug: false,
            start_func: None,
            wasm_binary: fs::read(path_prefix).unwrap(),
        };

        let res = futures::executor::block_on(runtime.start_runtime(vm_config, memory_adapter, host_adapters)).unwrap();

        RuntimeJobResult { vm_result: res }
    }
}
