mod cli;
use cli::{CliOptions, NearCliBackend};
mod errors;
use errors::Result;

fn main() -> Result<()> {
    CliOptions::handle::<NearCliBackend>()
}
