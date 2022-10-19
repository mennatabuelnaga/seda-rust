use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
/// The configuration for the seda engine.
pub struct Config {}

impl Config {
    /// For reading from a toml file.
    pub fn from_read<R: std::io::Read>(buf: &mut R) -> Result<Self, Box<dyn Error>> {
        let mut content = String::new();
        buf.read_to_string(&mut content)?;
        Ok(toml::from_str(&content)?)
    }

    /// For reading from a toml file from a path.
    pub fn read_from_path<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let mut file = std::fs::OpenOptions::new().read(true).open(path)?;
        Self::from_read(&mut file)
    }

    /// For writing a default configuration file.
    pub fn write_template<W: std::io::Write>(buf: &mut W) -> Result<(), Box<dyn Error>> {
        let template = Self::default();
        let content = toml::to_string_pretty(&template)?;
        buf.write_all(content.as_bytes())?;
        Ok(())
    }

    /// For creating a default config to a given path.
    pub fn create_template_from_path<P: AsRef<std::path::Path>>(path: P) -> Result<(), Box<dyn Error>> {
        let mut file = std::fs::OpenOptions::new().create(true).write(true).open(path)?;
        Self::write_template(&mut file)
    }
}
