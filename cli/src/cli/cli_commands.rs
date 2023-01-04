use jsonrpsee::{
    core::{client::ClientT, rpc_params},
    ws_client::WsClientBuilder,
};
use seda_config::CONFIG;

use crate::Result;

pub async fn call_cli(args: &[String]) -> Result<()> {
    let config = CONFIG.read().await;
    let seda_server_url = &config.seda_server_url;

    let client = WsClientBuilder::default().build(&seda_server_url).await?;
    let response: Vec<String> = client.request("cli", rpc_params![args]).await?;

    response.iter().for_each(|s| print!("{s}"));

    Ok(())
}
