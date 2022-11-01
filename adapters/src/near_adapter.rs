use jsonrpsee_types::Params;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::{query::QueryResponseKind, transactions::TransactionInfo};
use near_primitives::{
    transaction::SignedTransaction,
    types::{BlockReference, Finality, FunctionArgs},
    views::{FinalExecutionStatus, QueryRequest},
};
use serde_json::{from_slice, json, Number};
use tokio::time;

use super::errors::{NearAdapterError, Result};

pub async fn call_change_method(signed_tx: SignedTransaction, server_addr: String) -> Result<FinalExecutionStatus> {
    let client = JsonRpcClient::connect(server_addr);

    let request = methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest {
        signed_transaction: signed_tx.clone(),
    };

    let sent_at = time::Instant::now();
    let tx_hash = client.call(request).await?;

    loop {
        let response = client
            .call(methods::tx::RpcTransactionStatusRequest {
                transaction_info: TransactionInfo::TransactionId {
                    hash:       tx_hash,
                    account_id: signed_tx.transaction.signer_id.clone(),
                },
            })
            .await;
        let received_at = time::Instant::now();
        let delta = (received_at - sent_at).as_secs();

        if delta > 60 {
            return Err(NearAdapterError::BadTransactionTimestamp);
        }

        match response {
            Err(err) => match err.handler_error() {
                Some(methods::tx::RpcTransactionError::UnknownTransaction { .. }) => {
                    time::sleep(time::Duration::from_secs(2)).await;
                    continue;
                }
                _ => return Err(NearAdapterError::CallChangeMethod(err.to_string())),
            },
            Ok(response) => {
                println!("response gotten after: {}s", delta);

                println!("response.status: {:#?}", response.status);

                return Ok(response.status);
            }
        }
    }
}

pub async fn call_view_method(
    contract_id: String,
    method_name: String,
    args: Vec<u8>,
    server_addr: String,
) -> Result<String> {
    let client = JsonRpcClient::connect(server_addr);

    let request = methods::query::RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request:         QueryRequest::CallFunction {
            account_id: contract_id.parse()?,
            method_name,
            args: FunctionArgs::from(args),
        },
    };

    let response = client.call(request).await?;

    if let QueryResponseKind::CallResult(ref result) = response.kind {
        Ok(from_slice::<String>(&result.result)?)
    } else {
        Err(NearAdapterError::CallViewMethod)
    }
}

pub async fn get_node_owner(params: Params<'_>) -> Result<String> {
    let method_name = "get_node_owner".to_string();
    let mut seq = params.sequence();

    let contract_id: String = seq
        .next()
        .map_err(|_| NearAdapterError::MissingParam("contract_id".to_string()))?;
    let node_id: Number = seq
        .next()
        .map_err(|_| NearAdapterError::MissingParam("node_id".to_string()))?;
    let server_addr: String = seq
        .next()
        .map_err(|_| NearAdapterError::MissingParam("server_addr".to_string()))?;

    let args = json!({"node_id": node_id.to_string()}).to_string().into_bytes();
    call_view_method(contract_id, method_name, args, server_addr).await
}

pub async fn get_node_socket_address(params: Params<'_>) -> Result<String> {
    let method_name = "get_node_socket_address".to_string();
    let mut seq = params.sequence();
    let contract_id: String = seq
        .next()
        .map_err(|_| NearAdapterError::MissingParam("contract_id".to_string()))?;
    let node_id: Number = seq
        .next()
        .map_err(|_| NearAdapterError::MissingParam("node_id".to_string()))?;
    let server_addr: String = seq
        .next()
        .map_err(|_| NearAdapterError::MissingParam("server_addr".to_string()))?;

    let args = json!({"node_id": node_id.to_string()}).to_string().into_bytes();
    call_view_method(contract_id, method_name, args, server_addr).await
}

pub async fn get_nodes(params: Params<'_>) -> Result<String> {
    let method_name = "get_nodes".to_string();
    let mut seq = params.sequence();
    let contract_id: String = seq
        .next()
        .map_err(|_| NearAdapterError::MissingParam("contract_id".to_string()))?;
    let limit: Number = seq
        .next()
        .map_err(|_| NearAdapterError::MissingParam("limit".to_string()))?;
    let offset: Number = seq
        .next()
        .map_err(|_| NearAdapterError::MissingParam("offset".to_string()))?;
    let server_addr: String = seq
        .next()
        .map_err(|_| NearAdapterError::MissingParam("server_addr".to_string()))?;

    let args = json!({"limit": limit.to_string(), "offset": offset.to_string()})
        .to_string()
        .into_bytes();
    call_view_method(contract_id, method_name, args, server_addr).await
}

pub async fn register_node(params: Params<'_>) -> Result<FinalExecutionStatus> {
    let mut seq = params.sequence();
    let signed_tx: SignedTransaction = seq
        .next()
        .map_err(|_| NearAdapterError::MissingParam("signed_tx".to_string()))?;
    let server_addr: String = seq
        .next()
        .map_err(|_| NearAdapterError::MissingParam("server_addr".to_string()))?;

    call_change_method(signed_tx, server_addr).await
}

pub async fn remove_node(params: Params<'_>) -> Result<FinalExecutionStatus> {
    let mut seq = params.sequence();
    let signed_tx: SignedTransaction = seq
        .next()
        .map_err(|_| NearAdapterError::MissingParam("signed_tx".to_string()))?;
    let server_addr: String = seq
        .next()
        .map_err(|_| NearAdapterError::MissingParam("server_addr".to_string()))?;

    call_change_method(signed_tx, server_addr).await
}

pub async fn set_node_socket_address(params: Params<'_>) -> Result<FinalExecutionStatus> {
    let mut seq = params.sequence();
    let signed_tx: SignedTransaction = seq
        .next()
        .map_err(|_| NearAdapterError::MissingParam("signed_tx".to_string()))?;
    let server_addr: String = seq
        .next()
        .map_err(|_| NearAdapterError::MissingParam("server_addr".to_string()))?;

    call_change_method(signed_tx, server_addr).await
}
