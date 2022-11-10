mod cli;
use cli::{CliOptions, NearCliBackend};
mod errors;
use errors::*;
mod helpers;

#[tokio::main]
async fn main() -> Result<()> {
    CliOptions::handle::<NearCliBackend>().await
}
