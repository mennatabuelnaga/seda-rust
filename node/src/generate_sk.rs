use std::{fs::File, io::Write};

use bn254::PrivateKey;
use seda_config::NodeConfig;

pub fn generate_secret_key(config: NodeConfig) {
    // Checks if there is a SEDA_SECRET_KEY env variable
    // If not, it generates a new random one and saves it into a file
    if std::env::var("SEDA_SECRET_KEY").is_err() && config.seda_secret_key.is_empty() {
        let rng = &mut rand::thread_rng();
        let sk = PrivateKey::random(rng);
        let mut file = File::create(config.seda_secret_key_file_path.clone()).expect("Unable to create file");
        writeln!(file, "{:?}", sk.to_bytes().expect("couldn't serialize sk")).expect("Unable to write secret key");
    }
}
