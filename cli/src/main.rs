mod cli;
use cli::CliOptions;
mod errors;
use errors::Result;

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    // We hold the guards so logging works properly.
    let _guard = seda_logger::init();
    CliOptions::handle()
}
