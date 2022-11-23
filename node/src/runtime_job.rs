use std::{fs, path::PathBuf, sync::Arc};

use actix::{prelude::*, Handler, Message};
use futures::executor;
use parking_lot::Mutex;
use seda_runtime::{HostAdapters, InMemory, RunnableRuntime, Runtime, VmConfig, VmResult};

use crate::{
    event_queue::{Event, EventData},
    host::{DatabaseGet, Host},
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

pub struct RuntimeWorker {
    pub runtime: Option<Runtime>,
}

impl Actor for RuntimeWorker {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        let mut path_prefix = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path_prefix.push("./test_files/demo-cli.wasm");

        let mut runtime = Runtime::new();
        runtime.init(fs::read(path_prefix).unwrap()).unwrap();

        self.runtime = Some(runtime);
    }
}

impl Handler<RuntimeJob> for RuntimeWorker {
    type Result = RuntimeJobResult;

    fn handle(&mut self, msg: RuntimeJob, ctx: &mut Self::Context) -> Self::Result {
        let host_adapters = HostAdapters::<TestAdapters>::default();
        // TODO: Replace the binary with the actual consensus binary

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
        let x = ctx.address();
        let host_actor = Host::from_registry();

        let res =
            futures::executor::block_on(
                runtime.start_runtime(vm_config, memory_adapter, host_adapters, || async {
                    // match msg {
                    //     DatabaseGet => host_actor.send(DatabaseGet);
                    // }

                    let result = host_actor.send(DatabaseGet { key: "sd".to_string() }).await;

                    // let result = reqwest::get("https://swapi.dev/api/people/2/")
                    //     .await
                    //     .unwrap()
                    //     .text()
                    //     .await
                    //     .unwrap();

                    println!("YOOODODSODSOSODSOSDO {:?}", result);
                }),
            )
            .unwrap();

        // let handle = tokio::runtime::Handle::current();
        // let res = handle
        //     .block_on(runtime.start_runtime(vm_config, memory_adapter,
        // host_adapters))     .unwrap();

        // let res = runtime.start_runtime(vm_config, memory_adapter,
        // host_adapters).unwrap();

        RuntimeJobResult { vm_result: res }
    }
}

// #[derive(Message)]
// #[rtype(result = "String")]
// pub struct SayHello;

// impl Handler<SayHello> for RuntimeWorker {
//     type Result = String;

//     fn handle(&mut self, _msg: SayHello, ctx: &mut Self::Context) ->
// Self::Result {         println!("-----> Yelloooooww");
//         let res = futures::executor::block_on(reqwest::get("https://swapi.dev/api/people/2/"));

//         let host_addr = Host::from_registry();

//         // host_addr.do_send(Host::)

//         println!("----> Ressss: {:?}", res);

//         "-----> Yelloooooww".to_string()
//     }
// }
