use std::{fs::write, path::Path};

use bip39::{Language, Mnemonic, MnemonicType};
use seda_config::NodeConfig;

pub fn generate_mnemonic(config: NodeConfig) {
    // Checks if there is a SEDA_MNEMONIC env variable
    // If not, it generates a new random one and saves it into a file
    let mnemonic_path = config.seda_mnemonic_file_path.clone();
    if std::env::var("SEDA_MNEMONIC").is_err() && config.seda_mnemonic.is_empty() && !Path::new(&mnemonic_path).exists()
    {
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        let phrase = mnemonic.phrase();
        write(mnemonic_path, phrase).expect("Unable to write mnemonic");
    }
}
