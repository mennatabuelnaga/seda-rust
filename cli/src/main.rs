mod cli;
use cli::CliOptions;
mod errors;
use errors::Result;

fn main() -> Result<()> {
    // Load the dotenv file first since our config overloads values from it.
    dotenv::dotenv().ok();
    // Load the config before starting our logger.
    seda_config::create_and_load_or_load_config();
    // We hold the guards so logging works properly.
    let _guard = seda_logger::init();
    CliOptions::handle()
}
