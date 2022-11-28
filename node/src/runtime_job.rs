use std::{fs, path::PathBuf, sync::Arc};

use actix::{prelude::*, Handler, Message};
use parking_lot::Mutex;
use seda_runtime::{InMemory, RunnableRuntime, Runtime, VmConfig, VmResult};

use crate::{
    event_queue::{Event, EventData},
    runtime_adapter::RuntimeAdapter,
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

pub struct RuntimeWorker {
    pub runtime: Option<Runtime>,
}

impl Actor for RuntimeWorker {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        // TODO: Replace the binary condinationally with the consensus binary
        let mut path_prefix = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path_prefix.push("./test_files/demo-cli.wasm");

        let mut runtime = Runtime::new();
        runtime.init(fs::read(path_prefix).unwrap()).unwrap();

        self.runtime = Some(runtime);
    }
}

impl Handler<RuntimeJob> for RuntimeWorker {
    type Result = RuntimeJobResult;

    fn handle(&mut self, msg: RuntimeJob, _ctx: &mut Self::Context) -> Self::Result {
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
        };

        let runtime = self.runtime.as_ref().unwrap();

        let res =
            futures::executor::block_on(runtime.start_runtime::<RuntimeAdapter>(vm_config, memory_adapter)).unwrap();

        RuntimeJobResult { vm_result: res }
    }
}
