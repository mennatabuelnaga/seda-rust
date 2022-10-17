use clap::Parser;

#[derive(Parser)]
#[command(name = "sedad")]
#[command(author = "https://github.com/SedaProtocol")]
#[command(version = "0.0.1")]
#[command(about = "For interacting with the seda protocol.", long_about = None)]
struct Options {}

fn main() {
    let _options = Options::parse();
}
