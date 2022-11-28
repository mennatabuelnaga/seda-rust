use std::sync::Arc;

use near_crypto::InMemorySigner;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::{query::QueryResponseKind, transactions::TransactionInfo};
use near_primitives::{
    borsh::{BorshDeserialize, BorshSerialize},
    transaction::{Action, FunctionCallAction, SignedTransaction, Transaction},
    types::{AccountId, BlockReference, Finality, FunctionArgs},
    views::{FinalExecutionStatus, QueryRequest},
};
use seda_config::MainChainConfig;
use serde_json::from_slice;
use tokio::time;
use tracing::info;

use super::errors::{MainChainAdapterError, Result};
use crate::{MainChainAdapterTrait, TransactionParams};

#[derive(Debug)]
pub struct NearMainChain;

#[async_trait::async_trait]
impl MainChainAdapterTrait for NearMainChain {
    type Client = JsonRpcClient;
    type Config = MainChainConfig;
    type FinalExecutionStatus = near_primitives::views::FinalExecutionStatus;
    type SignedTransaction = near_primitives::transaction::SignedTransaction;

    fn new_client(config: &Self::Config) -> Result<Self::Client> {
        Ok(JsonRpcClient::connect(
            config
                .near_server_url
                .as_ref()
                .ok_or(MainChainAdapterError::MissingNearServerUrlConfig)?,
        ))
    }

    async fn construct_signed_tx(
        signer_acc_str: &str,
        signer_sk_str: &str,
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
        gas: u64,
        deposit: u128,
        server_url: &str,
    ) -> Result<Self::SignedTransaction> {
        let client = JsonRpcClient::connect(server_url);

        let signer_account_id: AccountId = signer_acc_str.parse()?;

        let signer_secret_key: near_crypto::SecretKey = signer_sk_str.parse()?;
        let signer = near_crypto::InMemorySigner::from_secret_key(signer_account_id, signer_secret_key);

        let access_key_query_response = client
            .call(methods::query::RpcQueryRequest {
                block_reference: BlockReference::latest(),
                request:         near_primitives::views::QueryRequest::ViewAccessKey {
                    account_id: signer.account_id.clone(),
                    public_key: signer.public_key.clone(),
                },
            })
            .await?;

        let current_nonce = match access_key_query_response.kind {
            QueryResponseKind::AccessKey(access_key) => access_key.nonce,
            _ => Err(MainChainAdapterError::FailedToExtractCurrentNonce)?,
        };

        let transaction = Transaction {
            signer_id:   signer.account_id.clone(),
            public_key:  signer.public_key.clone(),
            nonce:       current_nonce + 1,
            receiver_id: contract_id.parse()?,
            block_hash:  access_key_query_response.block_hash,
            actions:     vec![Action::FunctionCall(FunctionCallAction {
                method_name: method_name.to_string(),
                args,
                gas, // 100 TeraGas
                deposit,
            })],
        };
        let signed_transaction = transaction.sign(&signer);
        Ok(signed_transaction)
    }

    async fn sign_tx(client: Arc<Self::Client>, tx_params: TransactionParams) -> Result<SignedTransaction> {
        let signer_account_id: AccountId = tx_params.signer_acc_str.parse()?;

        let signer_secret_key: near_crypto::SecretKey = tx_params.signer_sk_str.parse()?;
        let signer = InMemorySigner::from_secret_key(signer_account_id, signer_secret_key);
        let access_key_query_response = client
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

    async fn send_tx(client: Arc<Self::Client>, signed_tx: SignedTransaction) -> Result<FinalExecutionStatus> {
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
                    info!("response gotten after: {}s", delta);

                    info!("response.status: {:#?}", response.status);

                    return Ok(response.status);
                }
            }
        }
    }

    async fn send_tx2(signed_tx: Vec<u8>, chain_server_address: &str) -> Result<Vec<u8>> {
        let signed_tx = SignedTransaction::try_from_slice(&signed_tx).expect("error deserializing signed txn");
        let request = methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest {
            signed_transaction: signed_tx.clone(),
        };

        let sent_at = time::Instant::now();
        let client = JsonRpcClient::connect(chain_server_address);
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

                    return Ok(response
                        .status
                        .try_to_vec()
                        .expect("error serializing FinalExecutionStatus"));
                }
            }
        }
    }

    async fn view(client: Arc<Self::Client>, contract_id: &str, method_name: &str, args: Vec<u8>) -> Result<String> {
        let request = methods::query::RpcQueryRequest {
            block_reference: BlockReference::Finality(Finality::Final),
            request:         QueryRequest::CallFunction {
                account_id:  contract_id.parse()?,
                method_name: method_name.to_string(),
                args:        FunctionArgs::from(args),
            },
        };

        let response = client.call(request).await?;

        if let QueryResponseKind::CallResult(ref result) = response.kind {
            Ok(from_slice::<String>(&result.result)?)
        } else {
            Err(MainChainAdapterError::CallViewMethod)
        }
    }

    async fn view2(contract_id: &str, method_name: &str, args: Vec<u8>, chain_server_address: &str) -> Result<String> {
        let request = methods::query::RpcQueryRequest {
            block_reference: BlockReference::Finality(Finality::Final),
            request:         QueryRequest::CallFunction {
                account_id:  contract_id.parse()?,
                method_name: method_name.to_string(),
                args:        FunctionArgs::from(args),
            },
        };
        let client = JsonRpcClient::connect(chain_server_address);

        let response = client.call(request).await?;

        if let QueryResponseKind::CallResult(ref result) = response.kind {
            Ok(from_slice::<String>(&result.result)?)
        } else {
            Err(MainChainAdapterError::CallViewMethod)
        }
    }



    async fn get_status_success(status: Vec<u8>) -> String {
        let x = FinalExecutionStatus::try_from_slice(&status).unwrap();
        match x {
            FinalExecutionStatus::SuccessValue(val) => serde_json::from_slice(&val).unwrap(),
            _ => "unknown".to_string(),
        }
    }
}
