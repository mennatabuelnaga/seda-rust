//! Defines a Chain type based on features when compiling.

mod another_chain;
pub use another_chain::AnotherChain;
use jsonrpsee_types::Params;
use seda_runtime_sdk::Chain;

mod errors;
use std::sync::Arc;

pub use errors::*;

mod chain_adapter_trait;
pub use chain_adapter_trait::*;

mod near_chain;
pub use near_chain::NearChain;

#[derive(Debug, Clone)]
pub enum Client {
    Another(Arc<()>),
    Near(Arc<near_jsonrpc_client::JsonRpcClient>),
}

impl Client {
    fn another(&self) -> Arc<()> {
        if let Self::Another(v) = self {
            v.clone()
        } else {
            unreachable!()
        }
    }

    fn near(&self) -> Arc<near_jsonrpc_client::JsonRpcClient> {
        if let Self::Near(v) = self {
            v.clone()
        } else {
            unreachable!()
        }
    }
}

pub mod chain {
    use super::*;

    async fn _sign_and_send_tx(chain: Chain, client: Client, tx_params: TransactionParams) -> Result<Vec<u8>> {
        let signed_tx = _sign_tx(chain, client.clone(), tx_params).await?;
        send_tx(chain, client, &signed_tx).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn construct_signed_tx(
        chain: Chain,
        signer_acc_str: &str,
        signer_sk_str: &str,
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
        gas: u64,
        deposit: u128,
        server_url: &str,
    ) -> Result<Vec<u8>> {
        match chain {
            Chain::Another => {
                AnotherChain::construct_signed_tx(
                    signer_acc_str,
                    signer_sk_str,
                    contract_id,
                    method_name,
                    args,
                    gas,
                    deposit,
                    server_url,
                )
                .await
            }
            Chain::Near => {
                NearChain::construct_signed_tx(
                    signer_acc_str,
                    signer_sk_str,
                    contract_id,
                    method_name,
                    args,
                    gas,
                    deposit,
                    server_url,
                )
                .await
            }
        }
    }

    async fn _sign_tx(chain: Chain, client: Client, tx_params: TransactionParams) -> Result<Vec<u8>> {
        match chain {
            Chain::Another => AnotherChain::sign_tx(client.another(), tx_params).await,
            Chain::Near => NearChain::sign_tx(client.near(), tx_params).await,
        }
    }

    pub async fn send_tx(chain: Chain, client: Client, signed_tx: &[u8]) -> Result<Vec<u8>> {
        match chain {
            Chain::Another => AnotherChain::send_tx(client.another(), signed_tx).await,
            Chain::Near => NearChain::send_tx(client.near(), signed_tx).await,
        }
    }

    pub async fn view(
        chain: Chain,
        client: Client,
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
    ) -> Result<String> {
        match chain {
            Chain::Another => AnotherChain::view(client.another(), contract_id, method_name, args).await,
            Chain::Near => NearChain::view(client.near(), contract_id, method_name, args).await,
        }
    }

    async fn _get_node_owner(chain: Chain, client: Client, params: Params<'_>) -> Result<String> {
        match chain {
            Chain::Another => AnotherChain::get_node_owner(client.another(), params).await,
            Chain::Near => NearChain::get_node_owner(client.near(), params).await,
        }
    }

    async fn _get_node_socket_address(chain: Chain, client: Client, params: Params<'_>) -> Result<String> {
        match chain {
            Chain::Another => AnotherChain::get_node_socket_address(client.another(), params).await,
            Chain::Near => NearChain::get_node_socket_address(client.near(), params).await,
        }
    }

    async fn _get_nodes(chain: Chain, client: Client, params: Params<'_>) -> Result<String> {
        match chain {
            Chain::Another => AnotherChain::get_nodes(client.another(), params).await,
            Chain::Near => NearChain::get_nodes(client.near(), params).await,
        }
    }

    async fn _register_node(chain: Chain, client: Client, params: Params<'_>) -> Result<Vec<u8>> {
        match chain {
            Chain::Another => AnotherChain::register_node(client.another(), params).await,
            Chain::Near => NearChain::register_node(client.near(), params).await,
        }
    }

    async fn _remove_node(chain: Chain, client: Client, params: Params<'_>) -> Result<Vec<u8>> {
        match chain {
            Chain::Another => AnotherChain::remove_node(client.another(), params).await,
            Chain::Near => NearChain::remove_node(client.near(), params).await,
        }
    }

    async fn _set_node_socket_address(chain: Chain, client: Client, params: Params<'_>) -> Result<Vec<u8>> {
        match chain {
            Chain::Another => AnotherChain::set_node_socket_address(client.another(), params).await,
            Chain::Near => NearChain::set_node_socket_address(client.near(), params).await,
        }
    }
}
