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

    /// For generating a default configuration file..
    pub fn template<W: std::io::Write>(buf: &mut W) -> Result<(), Box<dyn Error>> {
        let template = Self::default();
        let content = toml::to_string_pretty(&template)?;
        buf.write_all(content.as_bytes())?;
        Ok(())
    }
}
