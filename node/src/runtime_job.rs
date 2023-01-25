use std::{fs, path::PathBuf, sync::Arc};

use actix::{prelude::*, Handler, Message};
use futures::channel::mpsc::Sender;
use parking_lot::Mutex;
use seda_config::{ChainConfigs, NodeConfig};
use seda_runtime::{HostAdapter, InMemory, Result, RunnableRuntime, Runtime, VmConfig, VmResult};
use seda_runtime_sdk::{
    events::{Event, EventData},
    p2p::P2PCommand,
};
use tracing::info;

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
    pub runtime:                    Option<Runtime<HA>>,
    pub node_config:                NodeConfig,
    pub chain_configs:              ChainConfigs,
    pub p2p_command_sender_channel: Sender<P2PCommand>,
}

impl<HA: HostAdapter> Actor for RuntimeWorker<HA> {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        // TODO: Replace the binary conditionally with the consensus binary
        let mut path_prefix = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        #[cfg(debug_assertions)]
        path_prefix.push("../target/wasm32-wasi/debug/cli.wasm");
        #[cfg(not(debug_assertions))]
        path_prefix.push("../target/wasm32-wasi/release/cli.wasm");

        let node_config = self.node_config.clone();
        let chain_configs = self.chain_configs.clone();
        // TODO: when conditionally loading the consensus binary see if it allows full
        // or limited features
        let mut runtime =
            futures::executor::block_on(
                async move { Runtime::new(node_config, chain_configs, false).await.expect("TODO") },
            );

        runtime.init(fs::read(path_prefix).unwrap()).unwrap();

        self.runtime = Some(runtime);
    }
}

impl<HA: HostAdapter> Handler<RuntimeJob> for RuntimeWorker<HA> {
    type Result = Result<RuntimeJobResult>;

    fn handle(&mut self, msg: RuntimeJob, _ctx: &mut Self::Context) -> Self::Result {
        let memory_adapter = Arc::new(Mutex::new(InMemory::default()));

        let args: Vec<String> = match msg.event.data {
            EventData::ChainTick => vec![],
            EventData::CliCall(args) => args,
            // TODO: Make args accept byes only
            EventData::P2PMessage(message) => vec!["p2p".to_string(), String::from_utf8(message.data).unwrap()],
        };

        let vm_config = VmConfig {
            args,
            program_name: "test".to_string(),
            debug: false,
            start_func: None,
        };

        let runtime = self.runtime.as_ref().unwrap();

        let res = futures::executor::block_on(runtime.start_runtime(
            vm_config,
            memory_adapter,
            self.p2p_command_sender_channel.clone(),
        ))?;
        // TODO maybe set up a prettier log format rather than debug of this type?

        info!(vm_result = ?res);

        Ok(RuntimeJobResult { vm_result: res })
    }
}
