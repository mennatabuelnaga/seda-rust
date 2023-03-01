use std::{fs::File, io::Write};

use bip39::{Language, Mnemonic, MnemonicType};
use seda_config::NodeConfig;

pub fn generate_mnemonic(config: NodeConfig) {
    // Checks if there is a SEDA_MNEMONIC env variable
    // If not, it generates a new random one and saves it into a file
    if std::env::var("SEDA_MNEMONIC").is_err() && config.seda_mnemonic.is_empty() {
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        let phrase = mnemonic.phrase();
        let mut file = File::create(config.seda_mnemonic_file_path.clone()).expect("Unable to create file");
        writeln!(file, "{phrase:?}").expect("Unable to write mnemonic");
    }
}
