//! Defines a MainChain type based on features when compiling.

mod another_mainchain;
pub use another_mainchain::AnotherMainChain;
use jsonrpsee_types::Params;
use seda_config::AppConfig;
use seda_runtime_sdk::Chain;

mod errors;
use std::sync::Arc;

pub use errors::*;

mod mainchain_adapter_trait;
pub use mainchain_adapter_trait::*;

mod near_mainchain;
pub use near_mainchain::NearMainChain;

#[derive(Debug, Clone)]
pub enum Client {
    Another(Arc<()>),
    Near(Arc<near_jsonrpc_client::JsonRpcClient>),
}

impl Client {
    pub fn new(client: Chain, config: &AppConfig) -> Result<Self> {
        Ok(match client {
            Chain::Another => Self::Another(Arc::new(AnotherMainChain::new_client(
                config.another_chain.as_ref().expect("TODO remove when de-optioning"),
            )?)),
            Chain::Near => Self::Near(Arc::new(NearMainChain::new_client(
                config.near_chain.as_ref().expect("TODO remove when de-optioning"),
            )?)),
        })
    }

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
                AnotherMainChain::construct_signed_tx(
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
                NearMainChain::construct_signed_tx(
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
            Chain::Another => AnotherMainChain::sign_tx(client.another(), tx_params).await,
            Chain::Near => NearMainChain::sign_tx(client.near(), tx_params).await,
        }
    }

    pub async fn send_tx(chain: Chain, client: Client, signed_tx: &[u8]) -> Result<Vec<u8>> {
        match chain {
            Chain::Another => AnotherMainChain::send_tx(client.another(), signed_tx).await,
            Chain::Near => NearMainChain::send_tx(client.near(), signed_tx).await,
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
            Chain::Another => AnotherMainChain::view(client.another(), contract_id, method_name, args).await,
            Chain::Near => NearMainChain::view(client.near(), contract_id, method_name, args).await,
        }
    }

    async fn _get_node_owner(chain: Chain, client: Client, params: Params<'_>) -> Result<String> {
        match chain {
            Chain::Another => AnotherMainChain::get_node_owner(client.another(), params).await,
            Chain::Near => NearMainChain::get_node_owner(client.near(), params).await,
        }
    }

    async fn _get_node_socket_address(chain: Chain, client: Client, params: Params<'_>) -> Result<String> {
        match chain {
            Chain::Another => AnotherMainChain::get_node_socket_address(client.another(), params).await,
            Chain::Near => NearMainChain::get_node_socket_address(client.near(), params).await,
        }
    }

    async fn _get_nodes(chain: Chain, client: Client, params: Params<'_>) -> Result<String> {
        match chain {
            Chain::Another => AnotherMainChain::get_nodes(client.another(), params).await,
            Chain::Near => NearMainChain::get_nodes(client.near(), params).await,
        }
    }

    async fn _register_node(chain: Chain, client: Client, params: Params<'_>) -> Result<Vec<u8>> {
        match chain {
            Chain::Another => AnotherMainChain::register_node(client.another(), params).await,
            Chain::Near => NearMainChain::register_node(client.near(), params).await,
        }
    }

    async fn _remove_node(chain: Chain, client: Client, params: Params<'_>) -> Result<Vec<u8>> {
        match chain {
            Chain::Another => AnotherMainChain::remove_node(client.another(), params).await,
            Chain::Near => NearMainChain::remove_node(client.near(), params).await,
        }
    }

    async fn _set_node_socket_address(chain: Chain, client: Client, params: Params<'_>) -> Result<Vec<u8>> {
        match chain {
            Chain::Another => AnotherMainChain::set_node_socket_address(client.another(), params).await,
            Chain::Near => NearMainChain::set_node_socket_address(client.near(), params).await,
        }
    }
}
