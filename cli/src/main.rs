mod cli;
use cli::*;
mod errors;
use errors::*;
mod helpers;

use seda_adapters::MainChainAdapter;

#[tokio::main]
async fn main() -> Result<()> {
    Options::handle::<MainChainAdapter>().await
}
