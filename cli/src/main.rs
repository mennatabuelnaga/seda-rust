mod cli;
use cli::{CliBackend, CliOptions};
mod errors;
use errors::Result;

fn main() -> Result<()> {
    CliOptions::handle::<CliBackend>()
}
