use std::{fs, path::PathBuf, sync::Arc};

use actix::{prelude::*, Handler, Message};
use parking_lot::Mutex;
use seda_config::{ChainConfigs, NodeConfig};
use seda_runtime::{Result, RunnableRuntime, Runtime, VmConfig, VmResult};
use seda_runtime_adapters::{HostAdapter, InMemory};
use tracing::info;

use crate::event_queue::{Event, EventData};

#[derive(MessageResponse)]
pub struct RuntimeJobResult {
    pub vm_result: VmResult,
}

#[derive(Message)]
#[rtype(result = "Result<RuntimeJobResult>")]
pub struct RuntimeJob {
    pub event: Event,
}

pub struct RuntimeWorker<HA: HostAdapter> {
    pub runtime:       Option<Runtime<HA>>,
    pub node_config:   NodeConfig,
    pub chain_configs: ChainConfigs,
}

impl<HA: HostAdapter> Actor for RuntimeWorker<HA> {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        // TODO: Replace the binary condinationally with the consensus binary
        let mut path_prefix = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        #[cfg(debug_assertions)]
        path_prefix.push("../target/wasm32-wasi/debug/cli.wasm");
        #[cfg(not(debug_assertions))]
        path_prefix.push("../target/wasm32-wasi/release/cli.wasm");

        let node_config = self.node_config.clone();
        let chain_configs = self.chain_configs.clone();
        let mut runtime =
            futures::executor::block_on(async move { Runtime::new(node_config, chain_configs).await.expect("TODO") });

        runtime.init(fs::read(path_prefix).unwrap()).unwrap();

        self.runtime = Some(runtime);
    }
}

impl<HA: HostAdapter> Handler<RuntimeJob> for RuntimeWorker<HA> {
    type Result = Result<RuntimeJobResult>;

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

        let res = futures::executor::block_on(runtime.start_runtime(vm_config, memory_adapter))?;
        // TODO maybe set up a prettier log format rather than debug of this type?
        info!(vm_result = ?res);

        Ok(RuntimeJobResult { vm_result: res })
    }
}
