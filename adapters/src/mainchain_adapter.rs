// use core::num::flt2dec::Sign;

use async_trait;
use jsonrpsee_types::Params;
use near_crypto::InMemorySigner;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::{query::QueryResponseKind, transactions::TransactionInfo};
use near_primitives::{
    transaction::{Action, FunctionCallAction, SignedTransaction, Transaction},
    types::{AccountId, BlockReference, Finality, FunctionArgs},
    views::{FinalExecutionStatus, QueryRequest},
};
use serde_json::{from_slice, json, Number};
use tokio::time;

use super::errors::{MainChainAdapterError, Result};

pub struct TransactionParams {
    signer_acc_str: String,
    signer_sk_str:  String,
    contract_id:    String,
    method_name:    String,
    args:           Vec<u8>,
    gas:            u64,
    deposit:        u128,
}

#[derive(Clone, Debug)]
pub struct MainChainAdapter {
    client: JsonRpcClient,
}

#[async_trait::async_trait]
pub trait MainChainAdapterTrait {
    fn init(&mut self, rpc_endpoint: String);
    async fn sign_and_send_tx(&self, tx_params: TransactionParams) -> Result<FinalExecutionStatus>;
    async fn sign_tx(&self, tx_params: TransactionParams) -> Result<SignedTransaction>;
    async fn send_tx(&self, signed_tx: SignedTransaction) -> Result<FinalExecutionStatus>;
    async fn view(&self, contract_id: String, method_name: String, args: Vec<u8>) -> Result<String>;

    async fn get_node_owner(&self, params: Params<'_>) -> Result<String>;
    async fn get_node_socket_address(&self, params: Params<'_>) -> Result<String>;
    async fn get_nodes(&self, params: Params<'_>) -> Result<String>;
    async fn register_node(&self, params: Params<'_>) -> Result<FinalExecutionStatus>;
    async fn remove_node(&self, params: Params<'_>) -> Result<FinalExecutionStatus>;
    async fn set_node_socket_address(&self, params: Params<'_>) -> Result<FinalExecutionStatus>;
}

#[async_trait::async_trait]
impl MainChainAdapterTrait for MainChainAdapter {
    fn init(&mut self, rpc_endpoint: String) {
        self.client = JsonRpcClient::connect(rpc_endpoint);
    }

    async fn sign_and_send_tx(&self, tx_params: TransactionParams) -> Result<FinalExecutionStatus> {
        let signed_tx = self.sign_tx(tx_params).await?;
        self.send_tx(signed_tx).await
    }

    async fn sign_tx(&self, tx_params: TransactionParams) -> Result<SignedTransaction> {
        let signer_account_id: AccountId = tx_params.signer_acc_str.parse()?;

        let signer_secret_key: near_crypto::SecretKey = tx_params.signer_sk_str.parse()?;
        let signer = near_crypto::InMemorySigner::from_secret_key(signer_account_id, signer_secret_key);
        let access_key_query_response = self
            .client
            .call(methods::query::RpcQueryRequest {
                block_reference: BlockReference::latest(),
                request:         near_primitives::views::QueryRequest::ViewAccessKey {
                    account_id: signer.account_id.clone(),
                    public_key: signer.public_key.clone(),
                },
            })
            .await?;

        let nonce = match access_key_query_response.kind {
            QueryResponseKind::AccessKey(access_key) => access_key.nonce,
            _ => Err(MainChainAdapterError::FailedToExtractCurrentNonce)?,
        };

        let transaction = Transaction {
            signer_id:   signer.account_id.clone(),
            public_key:  signer.public_key.clone(),
            nonce:       nonce + 1,
            receiver_id: tx_params.contract_id.parse()?,
            block_hash:  access_key_query_response.block_hash,
            actions:     vec![Action::FunctionCall(FunctionCallAction {
                method_name: tx_params.method_name,
                args:        tx_params.args,
                gas:         tx_params.gas, // 100 TeraGas
                deposit:     tx_params.deposit,
            })],
        };
        let signed_transaction = transaction.sign(&signer);
        Ok(signed_transaction)
    }

    async fn send_tx(&self, signed_tx: SignedTransaction) -> Result<FinalExecutionStatus> {
        let request = methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest {
            signed_transaction: signed_tx.clone(),
        };

        let sent_at = time::Instant::now();
        let tx_hash = self.client.call(request).await?;

        loop {
            let response = self
                .client
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
                return Err(MainChainAdapterError::BadTransactionTimestamp);
            }

            match response {
                Err(err) => match err.handler_error() {
                    Some(methods::tx::RpcTransactionError::UnknownTransaction { .. }) => {
                        time::sleep(time::Duration::from_secs(2)).await;
                        continue;
                    }
                    _ => return Err(MainChainAdapterError::CallChangeMethod(err.to_string())),
                },
                Ok(response) => {
                    println!("response gotten after: {}s", delta);

                    println!("response.status: {:#?}", response.status);

                    return Ok(response.status);
                }
            }
        }
    }

    async fn view(&self, contract_id: String, method_name: String, args: Vec<u8>) -> Result<String> {
        let request = methods::query::RpcQueryRequest {
            block_reference: BlockReference::Finality(Finality::Final),
            request:         QueryRequest::CallFunction {
                account_id: contract_id.parse()?,
                method_name,
                args: FunctionArgs::from(args),
            },
        };

        let response = self.client.call(request).await?;

        if let QueryResponseKind::CallResult(ref result) = response.kind {
            Ok(from_slice::<String>(&result.result)?)
        } else {
            Err(MainChainAdapterError::CallViewMethod)
        }
    }

    async fn get_node_owner(&self, params: Params<'_>) -> Result<String> {
        Err(MainChainAdapterError::CallViewMethod)
    }

    async fn get_node_socket_address(&self, params: Params<'_>) -> Result<String> {
        Err(MainChainAdapterError::CallViewMethod)
    }

    async fn get_nodes(&self, params: Params<'_>) -> Result<String> {
        Err(MainChainAdapterError::CallViewMethod)
    }

    async fn register_node(&self, params: Params<'_>) -> Result<FinalExecutionStatus> {
        Err(MainChainAdapterError::CallViewMethod)
    }

    async fn remove_node(&self, params: Params<'_>) -> Result<FinalExecutionStatus> {
        Err(MainChainAdapterError::CallViewMethod)
    }

    async fn set_node_socket_address(&self, params: Params<'_>) -> Result<FinalExecutionStatus> {
        Err(MainChainAdapterError::CallViewMethod)
    }
}

// pub async fn get_node_owner(params: Params<'_>) -> Result<String> {
//     let method_name = "get_node_owner".to_string();
//     let mut seq = params.sequence();

//     let contract_id: String = seq
//         .next()
//         .map_err(|_|
// MainChainAdapterError::MissingParam("contract_id".to_string()))?;     let
// node_id: Number = seq         .next()
//         .map_err(|_|
// MainChainAdapterError::MissingParam("node_id".to_string()))?;
//     let server_addr: String = seq
//         .next()
//         .map_err(|_|
// MainChainAdapterError::MissingParam("server_addr".to_string()))?;

//     let args = json!({"node_id":
// node_id.to_string()}).to_string().into_bytes();
//     call_view_method(contract_id, method_name, args, server_addr).await
// }

// pub async fn get_node_socket_address(params: Params<'_>) -> Result<String> {
//     let method_name = "get_node_socket_address".to_string();
//     let mut seq = params.sequence();
//     let contract_id: String = seq
//         .next()
//         .map_err(|_|
// MainChainAdapterError::MissingParam("contract_id".to_string()))?;     let
// node_id: Number = seq         .next()
//         .map_err(|_|
// MainChainAdapterError::MissingParam("node_id".to_string()))?;
//     let server_addr: String = seq
//         .next()
//         .map_err(|_|
// MainChainAdapterError::MissingParam("server_addr".to_string()))?;

//     let args = json!({"node_id":
// node_id.to_string()}).to_string().into_bytes();
//     call_view_method(contract_id, method_name, args, server_addr).await
// }

// pub async fn get_nodes(params: Params<'_>) -> Result<String> {
//     let method_name = "get_nodes".to_string();
//     let mut seq = params.sequence();
//     let contract_id: String = seq
//         .next()
//         .map_err(|_|
// MainChainAdapterError::MissingParam("contract_id".to_string()))?;     let
// limit: Number = seq         .next()
//         .map_err(|_|
// MainChainAdapterError::MissingParam("limit".to_string()))?;     let offset:
// Number = seq         .next()
//         .map_err(|_|
// MainChainAdapterError::MissingParam("offset".to_string()))?;
//     let server_addr: String = seq
//         .next()
//         .map_err(|_|
// MainChainAdapterError::MissingParam("server_addr".to_string()))?;

//     let args = json!({"limit": limit.to_string(), "offset":
// offset.to_string()})         .to_string()
//         .into_bytes();
//     call_view_method(contract_id, method_name, args, server_addr).await
// }

// pub async fn register_node(params: Params<'_>) ->
// Result<FinalExecutionStatus> {     let mut seq = params.sequence();
//     let signed_tx: SignedTransaction = seq
//         .next()
//         .map_err(|_|
// MainChainAdapterError::MissingParam("signed_tx".to_string()))?;
//     let server_addr: String = seq
//         .next()
//         .map_err(|_|
// MainChainAdapterError::MissingParam("server_addr".to_string()))?;

//     call_change_method(signed_tx, server_addr).await
// }

// pub async fn remove_node(params: Params<'_>) -> Result<FinalExecutionStatus>
// {     let mut seq = params.sequence();
//     let signed_tx: SignedTransaction = seq
//         .next()
//         .map_err(|_|
// MainChainAdapterError::MissingParam("signed_tx".to_string()))?;
//     let server_addr: String = seq
//         .next()
//         .map_err(|_|
// MainChainAdapterError::MissingParam("server_addr".to_string()))?;

//     call_change_method(signed_tx, server_addr).await
// }

// pub async fn set_node_socket_address(params: Params<'_>) ->
// Result<FinalExecutionStatus> {     let mut seq = params.sequence();
//     let signed_tx: SignedTransaction = seq
//         .next()
//         .map_err(|_|
// MainChainAdapterError::MissingParam("signed_tx".to_string()))?;
//     let server_addr: String = seq
//         .next()
//         .map_err(|_|
// MainChainAdapterError::MissingParam("server_addr".to_string()))?;

//     call_change_method(signed_tx, server_addr).await
// }
