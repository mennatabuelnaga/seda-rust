use std::{future, net::SocketAddr, sync::Arc};

use actix::prelude::*;
use futures::FutureExt;
use jsonrpsee::{
    core::{async_trait, Error},
    proc_macros::rpc,
    types::Params,
};
use jsonrpsee_ws_server::{RpcModule, WsServerBuilder, WsServerHandle};
use seda_adapters::near_adapter::{
    get_node_owner,
    get_node_socket_address,
    get_nodes,
    register_node,
    remove_node,
    set_node_socket_address,
};
use tokio::runtime::Builder;

use crate::{
    app::App,
    event_queue::{Event, EventData},
    event_queue_handler::AddEventToQueue,
    runtime_job::{RuntimeJob, RuntimeWorker},
};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Stop;

impl Handler<Stop> for JsonRpcServer {
    type Result = ();

    fn handle(&mut self, _msg: Stop, _ctx: &mut Context<Self>) {
        // self.handle.to_owned().stop().unwrap().into_actor(self).wait(_ctx);
        println!("JsonRpcServer stopped!");
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

pub struct RpcServerTwo {
    app:            Addr<App>,
    runtime_worker: Addr<RuntimeWorker>,
}

impl RpcServerTwo {
    pub fn new(app: Addr<App>, runtime_worker: Addr<RuntimeWorker>) -> Self {
        Self { app, runtime_worker }
    }
}

#[async_trait]
impl RpcServer for RpcServerTwo {
    async fn cli(&self, args: Vec<String>) -> Result<Vec<String>, Error> {
        println!("{:?}", &args);
        // let received: Vec<String> = args;

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

        // self.app.send(msg).await.unwrap();
        Ok(result.vm_result.output)
    }
}

impl JsonRpcServer {
    pub async fn build(app: Addr<App>, runtime_worker: Addr<RuntimeWorker>) -> Result<Self, Error> {
        let server = WsServerBuilder::default().build("127.0.0.1:12345").await?;
        let rpc = RpcServerTwo::new(app, runtime_worker);

        server.start(rpc.into_rpc())?;
        Ok(Self {})
    }
}

impl Actor for JsonRpcServer {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("JsonRpcServer starting...");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("JsonRpcServer stopped");
    }
}
