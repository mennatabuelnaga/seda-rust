mod cli;
use cli::{CliOptions, NearCliBackend};
mod errors;
use errors::*;
mod helpers;

fn main() -> Result<()> {
    CliOptions::handle::<NearCliBackend>()
}
