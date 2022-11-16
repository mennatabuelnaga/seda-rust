use std::sync::Arc;

use actix::prelude::*;
use jsonrpsee_core::Error;
use jsonrpsee_ws_server::{RpcModule, WsServerBuilder, WsServerHandle};
use seda_adapters::near_adapter::{
    get_node_owner,
    get_node_socket_address,
    get_nodes,
    register_node,
    remove_node,
    set_node_socket_address,
};

use crate::{
    app::App,
    event_queue::{Event, EventData},
    event_queue_handler::AddEventToQueue,
};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Stop;

impl Handler<Stop> for JsonRpcServer {
    type Result = ();

    fn handle(&mut self, _msg: Stop, _ctx: &mut Context<Self>) {
        self.handle.to_owned().stop().unwrap().into_actor(self).wait(_ctx);
        println!("JsonRpcServer stopped!");
    }
}

pub struct JsonRpcServer {
    handle: WsServerHandle,
}

impl JsonRpcServer {
    pub async fn build(app: Arc<Addr<App>>) -> Result<Self, Error> {
        let mut module = RpcModule::new(());
        // TODO: refactor module configuration

        // register view methods
        module.register_async_method("cli", |params, _| async move {
            println!("hey");
            // let status = get_node_socket_address(params).await;
            // status.map_err(|err|
            // jsonrpsee_core::Error::Custom(err.to_string()))
            app.send(AddEventToQueue {
                event: Event {
                    data: EventData::MainChainTick,
                    id:   "test".to_string(),
                },
            })
            .await
            .unwrap();
            Ok("This works")
        })?;

        module.register_async_method("get_node_socket_address", |params, _| async move {
            let status = get_node_socket_address(params).await;
            status.map_err(|err| jsonrpsee_core::Error::Custom(err.to_string()))
        })?;

        module.register_async_method("get_node_owner", |params, _| async move {
            let status = get_node_owner(params).await;
            status.map_err(|err| jsonrpsee_core::Error::Custom(err.to_string()))
        })?;

        module.register_async_method("get_nodes", |params, _| async move {
            let status = get_nodes(params).await;
            status.map_err(|err| jsonrpsee_core::Error::Custom(err.to_string()))
        })?;

        // register change methods

        module.register_async_method("register_node", |params, _| async move {
            let result = register_node(params).await;
            result.map_err(|err| jsonrpsee_core::Error::Custom(err.to_string()))
        })?;

        module.register_async_method("remove_node", |params, _| async move {
            let result = remove_node(params).await;
            result.map_err(|err| jsonrpsee_core::Error::Custom(err.to_string()))
        })?;

        module.register_async_method("set_node_socket_address", |params, _| async move {
            let result = set_node_socket_address(params).await;
            result.map_err(|err| jsonrpsee_core::Error::Custom(err.to_string()))
        })?;

        let server = WsServerBuilder::default().build("127.0.0.1:12345").await?;

        let handle = server.start(module)?;

        Ok(Self { handle })
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
