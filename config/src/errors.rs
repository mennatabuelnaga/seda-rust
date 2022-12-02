use thiserror::Error;

#[derive(Error, Debug)]
pub enum TomlError {
    #[error("Invalid Toml Deserialization: {0}")]
    Deserialize(#[from] toml::de::Error),
    #[error("Invalid Toml Serialization: {0}")]
    Serialize(#[from] toml::ser::Error),
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Config io error: {0}")]
    ConfigIoError(#[from] std::io::Error),
    #[error(transparent)]
    InvalidTomlConfig(#[from] TomlError),
}

pub type Result<T, E = ConfigError> = core::result::Result<T, E>;
