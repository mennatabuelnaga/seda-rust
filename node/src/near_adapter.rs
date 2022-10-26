use std::{error::Error, fmt};

use error_stack::{Report, Result};
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::{query::QueryResponseKind, transactions::TransactionInfo};
use near_primitives::{
    transaction::SignedTransaction,
    types::{BlockReference, Finality, FunctionArgs},
    views::{FinalExecutionStatus, QueryRequest},
};
use serde_json::from_slice;
use tokio::time;

#[derive(Debug)]
pub struct CallChangeMethodError;

impl fmt::Display for CallChangeMethodError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Error calling contract change method")
    }
}

impl Error for CallChangeMethodError {}

#[derive(Debug)]
pub struct CallViewMethodError;

impl fmt::Display for CallViewMethodError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Error calling contract view method")
    }
}

impl Error for CallViewMethodError {}

pub async fn call_change_method(
    signed_tx: SignedTransaction,
    server_addr: String,
) -> Result<FinalExecutionStatus, CallChangeMethodError> {
    let client = JsonRpcClient::connect(server_addr);

    let request = methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest {
        signed_transaction: signed_tx.clone(),
    };

    let sent_at = time::Instant::now();
    let tx_hash = client.call(request).await.expect("Couldn't fetch tx hash");

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
            // Err("time limit exceeded for the transaction to be recognized")?;
            Err(Report::new(CallChangeMethodError)
                .attach_printable("time limit exceeded for the transaction to be recognized".to_string()))?;
        }

        match response {
            Err(err) => match err.handler_error() {
                Some(methods::tx::RpcTransactionError::UnknownTransaction { .. }) => {
                    time::sleep(time::Duration::from_secs(2)).await;
                    continue;
                }
                _ => Err(Report::new(CallChangeMethodError).attach_printable(format!("{:?}", err)))?,
            },
            Ok(response) => {
                println!("response gotten after: {}s", delta);

                println!("response.status: {:#?}", response.status);

                return Ok(response.status);
                // break;
            }
        }
    }
}

pub async fn call_view_method(
    contract_id: String,
    method_name: String,
    args: Vec<u8>,
    server_addr: String,
) -> Result<String, CallViewMethodError> {
    let client = JsonRpcClient::connect(server_addr);

    let request = methods::query::RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request:         QueryRequest::CallFunction {
            account_id: contract_id.parse().unwrap(),
            method_name,
            args: FunctionArgs::from(args),
        },
    };

    let response = client.call(request).await.unwrap();

    if let QueryResponseKind::CallResult(ref result) = response.kind {
        match from_slice::<String>(&result.result).is_ok() {
            true => Ok(from_slice::<String>(&result.result).unwrap()),
            false => {
                Err(Report::new(CallViewMethodError)
                    .attach_printable("Couldn't deserialize status to string".to_string()))
            }
        }
    } else {
        Err(Report::new(CallViewMethodError).attach_printable("Couldn't fetch status".to_string()))
    }
}
