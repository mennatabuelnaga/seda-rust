use std::{fs, path::PathBuf};

use actix::{prelude::*, Handler, Message};
use seda_runtime::{HostAdapters, RunnableRuntime, Runtime, VmConfig};

use crate::{event_queue::Event, test_adapters::TestAdapters};

#[derive(Message)]
#[rtype(result = "()")]
pub struct RuntimeJob {
    pub event: Event,
}

pub struct RuntimeWorker;

impl Actor for RuntimeWorker {
    type Context = SyncContext<Self>;
}

impl Handler<RuntimeJob> for RuntimeWorker {
    type Result = ();

    fn handle(&mut self, _msg: RuntimeJob, _ctx: &mut Self::Context) -> Self::Result {
        let host_adapters = HostAdapters::<TestAdapters>::default();
        // TODO: Replace the binary with the actual consensus binary
        let mut path_prefix = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path_prefix.push("./test_files/promise-wasm-bin.wasm");

        let runtime = Runtime {};

        let vm_config = VmConfig {
            args:         vec![],
            program_name: "test".to_string(),
            debug:        false,
            start_func:   None,
            wasm_binary:  fs::read(path_prefix).unwrap(),
        };

        let _res = futures::executor::block_on(runtime.start_runtime(vm_config, host_adapters));
    }
}
