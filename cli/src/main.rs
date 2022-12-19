mod cli;
use cli::CliOptions;
mod errors;
use errors::Result;

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    seda_logger::init();
    CliOptions::handle()
}
