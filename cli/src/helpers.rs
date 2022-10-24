use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::transaction::SignedTransaction;
use near_primitives::transaction::{Action, FunctionCallAction, Transaction};
use near_primitives::types::{AccountId, BlockReference};

#[allow(clippy::too_many_arguments)]
pub async fn construct_signed_tx(
    signer_acc_str: String,
    signer_sk_str: String,
    contract_id: String,
    method_name: String,
    args: Vec<u8>,
    gas: u64,
    deposit: u128,
    server_url: String,
) -> Result<SignedTransaction, Box<dyn std::error::Error>> {
    let client = JsonRpcClient::connect(server_url);

    let signer_account_id: AccountId = signer_acc_str.parse().unwrap();

    let signer_secret_key: near_crypto::SecretKey = signer_sk_str.parse().unwrap();
    let signer = near_crypto::InMemorySigner::from_secret_key(signer_account_id, signer_secret_key);

    let access_key_query_response = client
        .call(methods::query::RpcQueryRequest {
            block_reference: BlockReference::latest(),
            request: near_primitives::views::QueryRequest::ViewAccessKey {
                account_id: signer.account_id.clone(),
                public_key: signer.public_key.clone(),
            },
        })
        .await?;

    let current_nonce = match access_key_query_response.kind {
        QueryResponseKind::AccessKey(access_key) => access_key.nonce,
        _ => Err("failed to extract current nonce")?,
    };

    let transaction = Transaction {
        signer_id: signer.account_id.clone(),
        public_key: signer.public_key.clone(),
        nonce: current_nonce + 1,
        receiver_id: contract_id.parse().unwrap(),
        block_hash: access_key_query_response.block_hash,
        actions: vec![Action::FunctionCall(FunctionCallAction {
            method_name,
            args,
            gas, // 100 TeraGas
            deposit,
        })],
    };
    let signed_transaction = transaction.sign(&signer);
    Ok(signed_transaction)
}
