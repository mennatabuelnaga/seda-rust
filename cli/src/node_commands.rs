use jsonrpsee::{core::client::ClientT, rpc_params, ws_client::WsClientBuilder};

use super::errors::{get_env_var, Result};

#[tokio::main]
pub async fn call_cli(args: Vec<String>) -> Result<Vec<String>> {
    let seda_server_url = get_env_var("SEDA_SERVER_URL")?;

    let client = WsClientBuilder::default().build(&seda_server_url).await?;

    let response: Vec<String> = client.request("cli", rpc_params![args]).await?;

    // Output the CLI lines
    for item in response.iter() {
        print!("{item}");
    }

    Ok(response)
}
