use std::sync::Arc;

use jsonrpsee_types::Params;
use near_primitives::{transaction::SignedTransaction, views::FinalExecutionStatus};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{MainChainAdapterError, Result};

pub struct TransactionParams {
    pub signer_acc_str: String,
    pub signer_sk_str:  String,
    pub contract_id:    String,
    pub method_name:    String,
    pub args:           Vec<u8>,
    pub gas:            u64,
    pub deposit:        u128,
}

#[derive(Deserialize)]
pub struct Node {
    pub contract_id: usize,
    pub node_id:     usize,
}

#[derive(Deserialize)]
pub struct NodeDetails {
    pub contract_id: usize,
    pub limit:       usize,
    pub offset:      usize,
}

#[async_trait::async_trait]
pub trait MainChainAdapterTrait: Send + Sync {
    type Client: Send + Sync + 'static;
    fn new_client(server_addr: &str) -> Self::Client;

    type SignedTransaction: Serialize;
    async fn construct_signed_tx(
        signer_acc_str: &str,
        signer_sk_str: &str,
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
        gas: u64,
        deposit: u128,
        server_url: &str,
    ) -> Result<Self::SignedTransaction>;
    // Some methods like this and a few others seemed
    // like they would never charge so we can have default
    // impls like this.
    async fn sign_and_send_tx(client: Arc<Self::Client>, tx_params: TransactionParams) -> Result<FinalExecutionStatus> {
        let signed_tx = Self::sign_tx(client.clone(), tx_params).await?;
        Self::send_tx(client, signed_tx).await
    }

    async fn sign_tx(client: Arc<Self::Client>, tx_params: TransactionParams) -> Result<SignedTransaction>;
    async fn send_tx(client: Arc<Self::Client>, signed_tx: SignedTransaction) -> Result<FinalExecutionStatus>;
    async fn view(client: Arc<Self::Client>, contract_id: String, method_name: &str, args: Vec<u8>) -> Result<String>;

    async fn get_node_owner(client: Arc<Self::Client>, params: Params<'_>) -> Result<String> {
        let method_name = "get_node_owner";
        // We can change node here to be generic if needed.
        // Just have to have a method to access contract id and node id.
        let params = params
            .one::<Node>()
            .map_err(|_| MainChainAdapterError::BadParams(method_name.to_string()))?;

        let args = json!({"node_id": params.node_id.to_string()}).to_string().into_bytes();

        Self::view(client, params.contract_id.to_string(), method_name, args).await
    }

    async fn get_node_socket_address(client: Arc<Self::Client>, params: Params<'_>) -> Result<String> {
        let method_name = "get_node_socket_address";
        let params = params
            .one::<Node>()
            .map_err(|_| MainChainAdapterError::BadParams(method_name.to_string()))?;

        let args = json!({"node_id": params.node_id.to_string()}).to_string().into_bytes();

        Self::view(client, params.contract_id.to_string(), method_name, args).await
    }

    async fn get_nodes(client: Arc<Self::Client>, params: Params<'_>) -> Result<String> {
        let method_name = "get_nodes";
        // We can change node here to be generic if needed.
        // Just have to have a method to access limit, offset, and contract id.
        let params = params
            .one::<NodeDetails>()
            .map_err(|_| MainChainAdapterError::BadParams(method_name.to_string()))?;

        let args = json!({"limit": params.limit.to_string(), "offset": params.offset.to_string()})
            .to_string()
            .into_bytes();

        Self::view(client, params.contract_id.to_string(), method_name, args).await
    }

    async fn register_node(client: Arc<Self::Client>, params: Params<'_>) -> Result<FinalExecutionStatus> {
        let signed_tx = params
            .one::<SignedTransaction>()
            .map_err(|_| MainChainAdapterError::BadParams("register_node".to_string()))?;

        Self::send_tx(client, signed_tx).await
    }

    async fn remove_node(client: Arc<Self::Client>, params: Params<'_>) -> Result<FinalExecutionStatus> {
        let signed_tx = params
            .one::<SignedTransaction>()
            .map_err(|_| MainChainAdapterError::BadParams("register_node".to_string()))?;

        Self::send_tx(client, signed_tx).await
    }

    async fn set_node_socket_address(client: Arc<Self::Client>, params: Params<'_>) -> Result<FinalExecutionStatus> {
        let signed_tx = params
            .one::<SignedTransaction>()
            .map_err(|_| MainChainAdapterError::BadParams("register_node".to_string()))?;

        Self::send_tx(client, signed_tx).await
    }
}
