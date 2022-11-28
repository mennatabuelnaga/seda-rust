use actix::prelude::*;
use jsonrpsee::{
    core::{async_trait, Error},
    proc_macros::rpc,
    server::{ServerBuilder, ServerHandle},
};

use crate::{
    event_queue::{Event, EventData},
    runtime_job::{RuntimeJob, RuntimeWorker},
};

#[rpc(server)]
pub trait Rpc {
    #[method(name = "cli")]
    async fn cli(&self, args: Vec<String>) -> Result<Vec<String>, Error>;
}

pub struct CliServer {
    runtime_worker: Addr<RuntimeWorker>,
}

#[async_trait]
impl RpcServer for CliServer {
    async fn cli(&self, args: Vec<String>) -> Result<Vec<String>, Error> {
        debug!("{:?}", &args);

        let result = self
            .runtime_worker
            .send(RuntimeJob {
                event: Event {
                    id:   "test".to_string(),
                    data: EventData::CliCall(args),
                },
            })
            .await
            .unwrap();

        Ok(result.vm_result.output)
    }
}

pub struct JsonRpcServer {
    // handle: WsServerHandle,
}

#[rpc(server)]
pub trait Rpc {
    #[method(name = "cli")]
    async fn cli(&self, args: Vec<String>) -> Result<Vec<String>, Error>;
}

pub struct CliServer {
    runtime_worker: Addr<RuntimeWorker>,
}

#[async_trait]
impl RpcServer for CliServer {
    async fn cli(&self, args: Vec<String>) -> Result<Vec<String>, Error> {
        println!("{:?}", &args);

        let result = self
            .runtime_worker
            .send(RuntimeJob {
                event: Event {
                    id:   "test".to_string(),
                    data: EventData::CliCall(args),
                },
            })
            .await
            .unwrap();

        Ok(result.vm_result.output)
    }
}

pub struct JsonRpcServer {
    handle: ServerHandle,
}

impl JsonRpcServer {
    pub async fn start(runtime_worker: Addr<RuntimeWorker>) -> Result<Self, Error> {
        let server = ServerBuilder::default().build("127.0.0.1:12345").await?;
        let rpc = CliServer { runtime_worker };
        let handle = server.start(rpc.into_rpc())?;

        Ok(Self { handle })
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        self.handle.clone().stop()?;

        Ok(())
    }
}
