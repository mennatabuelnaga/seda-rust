use actix::prelude::*;
use jsonrpsee_ws_server::{RpcModule, WsServerBuilder, WsServerHandle};
use seda_adapters::MainChainAdapterTrait;
use tracing::info;

use crate::Result;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Stop;

impl Handler<Stop> for JsonRpcServer {
    type Result = ();

    fn handle(&mut self, _msg: Stop, _ctx: &mut Context<Self>) {
        self.handle.to_owned().stop().unwrap().into_actor(self).wait(_ctx);
    }
}

// TODO genericize and make an adapter
pub struct JsonRpcServer {
    handle: WsServerHandle,
}

impl JsonRpcServer {
    pub async fn build<T: MainChainAdapterTrait>(main_chain_config: &T::Config, server_url: &str) -> Result<Self> {
        let mut module = RpcModule::new(T::new_client(main_chain_config)?);
        // TODO: refactor module configuration

        // register view methods
        module.register_async_method("get_node_socket_address", |params, ctx| async move {
            let status = T::get_node_socket_address(ctx, params).await;
            status.map_err(|err| jsonrpsee_core::Error::Custom(err.to_string()))
        })?;

        module.register_async_method("get_node_owner", |params, ctx| async move {
            let status = T::get_node_owner(ctx, params).await;
            status.map_err(|err| jsonrpsee_core::Error::Custom(err.to_string()))
        })?;

        module.register_async_method("get_nodes", |params, ctx| async move {
            let status = T::get_nodes(ctx, params).await;
            status.map_err(|err| jsonrpsee_core::Error::Custom(err.to_string()))
        })?;

        // register change methods
        module.register_async_method("register_node", |params, ctx| async move {
            let result = T::register_node(ctx, params).await;
            result.map_err(|err| jsonrpsee_core::Error::Custom(err.to_string()))
        })?;

        module.register_async_method("remove_node", |params, ctx| async move {
            let result = T::remove_node(ctx, params).await;
            result.map_err(|err| jsonrpsee_core::Error::Custom(err.to_string()))
        })?;

        module.register_async_method("set_node_socket_address", |params, ctx| async move {
            let result = T::set_node_socket_address(ctx, params).await;
            result.map_err(|err| jsonrpsee_core::Error::Custom(err.to_string()))
        })?;

        let server = WsServerBuilder::default().build(server_url).await?;

        let handle = server.start(module)?;

        Ok(Self { handle })
    }
}

impl Actor for JsonRpcServer {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("JsonRpcServer starting...");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("JsonRpcServer stopped");
    }
}
