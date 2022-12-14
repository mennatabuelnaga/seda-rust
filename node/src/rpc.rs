use actix::prelude::*;
use jsonrpsee::{
    core::{async_trait, Error},
    proc_macros::rpc,
};
use jsonrpsee_ws_server::{WsServerBuilder, WsServerHandle};
use seda_runtime_adapters::HostAdapter;
use tracing::debug;

use crate::{
    event_queue::{Event, EventData},
    runtime_job::{RuntimeJob, RuntimeWorker},
};

#[rpc(server)]
pub trait Rpc {
    #[method(name = "cli")]
    async fn cli(&self, args: Vec<String>) -> Result<Vec<String>, Error>;
}

pub struct CliServer<HA: HostAdapter> {
    runtime_worker: Addr<RuntimeWorker<HA>>,
}

#[async_trait]
impl<HA: HostAdapter> RpcServer for CliServer<HA> {
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
            .map_err(|err| Error::Custom(err.to_string()))?;

        Ok(result.map_err(|err| Error::Custom(err.to_string()))?.vm_result.output)
    }
}
pub struct JsonRpcServer {
    handle: WsServerHandle,
}

impl JsonRpcServer {
    pub async fn start<HA: HostAdapter>(runtime_worker: Addr<RuntimeWorker<HA>>) -> Result<Self, Error> {
        let server = WsServerBuilder::default().build("127.0.0.1:12345").await?;
        let rpc = CliServer { runtime_worker };
        let handle = server.start(rpc.into_rpc())?;
        info!("RPC Server listening on {}", addrs);

        Ok(Self { handle })
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        self.handle.clone().stop()?;

        Ok(())
    }
}
