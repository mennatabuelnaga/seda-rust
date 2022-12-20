use std::{fmt::Debug, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::Result;

pub struct TransactionParams {
    pub signer_acc_str: String,
    pub signer_sk_str:  String,
    pub contract_id:    String,
    pub method_name:    String,
    pub args:           Vec<u8>,
    pub gas:            u64,
    pub deposit:        u128,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "")]
pub struct NodeIds {
    pub contract_id: String,
    pub node_id:     u64,
}

#[derive(Serialize, Deserialize)]
pub struct NodeDetails {
    pub contract_id: String,
    pub limit:       u64,
    pub offset:      u64,
}

// TODO once rpc becomes a trait need to replace params type.
#[async_trait::async_trait]
pub trait MainChainAdapterTrait: Debug + Send + Sync + 'static {
    /// The Config fields for the adapter specific implementation.
    type Config: seda_config::Config + Send + Sync;
    /// The Client type for the adapter specific implementation.
    type Client: Send + Sync + 'static;

    /// Returns an new instance of the client given the server address.
    fn new_client(config: &Self::Config) -> Result<Self::Client>;

    /// Returns a signed transaction given the necessary information.
    #[allow(clippy::too_many_arguments)]
    async fn construct_signed_tx(
        signer_acc_str: &str,
        signer_sk_str: &str,
        contract_id: &str,
        method_name: &str,
        args: Vec<u8>,
        gas: u64,
        deposit: u128,
        server_url: &str,
    ) -> Result<Vec<u8>>;

    /// To send a transaction for the adapter specific implementation.
    async fn send_tx(client: Arc<Self::Client>, signed_tx: &[u8]) -> Result<Vec<u8>>;
    /// To view for the adapter specific implementation.
    async fn view(client: Arc<Self::Client>, contract_id: &str, method_name: &str, args: Vec<u8>) -> Result<Vec<u8>>;
}
