use std::{fs::write, path::Path};

use ed25519_dalek::SecretKey;
use rand::rngs::OsRng;
use seda_config::NodeConfig;
pub fn generate_sk(config: NodeConfig) {
    // Checks if there is a SEDA_SECRET_KEY env variable
    // If not, it generates a new random one and saves it into a file
    let sk_path = config.seda_sk_file_path.clone();
    if std::env::var("SEDA_SECRET_KEY").is_err() && config.seda_secret_key.is_empty() && !Path::new(&sk_path).exists() {
        let mut csprng = OsRng {};
        let sk = SecretKey::generate(&mut csprng);
        write(sk_path, hex::encode(&sk)).expect("Unable to write mnemonic");
    }
}
