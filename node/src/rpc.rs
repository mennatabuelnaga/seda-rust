use actix::prelude::*;
use jsonrpsee_ws_server::{RpcModule, WsServerBuilder, WsServerHandle};
use near_primitives::transaction::SignedTransaction;
use serde_json::{json, Number};

use crate::near_adapter::{call_change_method, call_view_method};

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
    pub async fn build() -> Self {
        let mut module = RpcModule::new(());
        // TODO: refactor module configuration

        // register view methods

        module
            .register_async_method("get_node_socket_address", |params, _| async move {
                let method_name = "get_node_socket_address".to_string();
                let mut seq = params.sequence();
                let contract_id: String = seq.next().expect("Contract Id must be set");
                let node_id: Number = seq.next().expect("Node Id must be set");
                let server_addr: String = seq.next().expect("Server address must be set");
                let args = json!({"node_id": node_id.to_string()}).to_string().into_bytes();
                let status = call_view_method(contract_id, method_name, args, server_addr)
                    .await
                    .unwrap();
                Ok(status)
            })
            .unwrap();

        module
            .register_async_method("get_node_owner", |params, _| async move {
                let method_name = "get_node_owner".to_string();
                let mut seq = params.sequence();
                let contract_id: String = seq.next().expect("Contract Id must be set");
                let node_id: Number = seq.next().expect("Node Id must be set");
                let server_addr: String = seq.next().expect("Server address must be set");
                let args = json!({"node_id": node_id.to_string()}).to_string().into_bytes();
                let status = call_view_method(contract_id, method_name, args, server_addr)
                    .await
                    .unwrap();
                Ok(status)
            })
            .unwrap();

        // register change methods

        module
            .register_async_method("register_node", |params, _| async move {
                let mut seq = params.sequence();
                let signed_tx: SignedTransaction = seq.next().expect("Signed tx must be set");
                let server_addr: String = seq.next().expect("Server address must be set");
                let result = call_change_method(signed_tx, server_addr).await.unwrap();
                Ok(result)
            })
            .unwrap();

        module
            .register_async_method("remove_node", |params, _| async move {
                let mut seq = params.sequence();
                let signed_tx: SignedTransaction = seq.next().expect("Signed tx must be set");
                let server_addr: String = seq.next().expect("Server address must be set");
                let result = call_change_method(signed_tx, server_addr).await.unwrap();
                Ok(result)
            })
            .unwrap();

        module
            .register_async_method("set_node_socket_address", |params, _| async move {
                let mut seq = params.sequence();
                let signed_tx: SignedTransaction = seq.next().expect("Signed tx must be set");
                let server_addr: String = seq.next().expect("Server address must be set");
                let result = call_change_method(signed_tx, server_addr).await.unwrap();
                Ok(result)
            })
            .unwrap();

        let server = WsServerBuilder::default()
            .build("127.0.0.1:12345")
            .await
            .expect("builder didnt work");

        let handle = server.start(module).expect("server should start");

        Self { handle }
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
