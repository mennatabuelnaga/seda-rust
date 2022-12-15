mod cli;
use cli::CliOptions;
mod errors;
use errors::Result;

fn main() -> Result<()> {
    CliOptions::handle()
}
