mod cli;
use cli::{CliOptions, NearCliBackend};
mod config;
mod errors;
use errors::Result;
mod helpers;

fn main() -> Result<()> {
    CliOptions::handle::<NearCliBackend>()
}
