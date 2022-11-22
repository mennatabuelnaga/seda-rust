mod cli;
use cli::{CliOptions, NearCliBackend};
mod config;
mod errors;
use errors::Result;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    seda_logger::init(|| Ok(CliOptions::handle::<NearCliBackend>()?))
}
