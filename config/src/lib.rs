use serde::{de::DeserializeOwned, Serialize};

#[macro_export]
macro_rules! env_overwrite {
    ($field:expr, $name:expr) => {
        if let Ok(var) = std::env::var($name) {
            $field = var;
        }
    };
}

pub trait Config: std::fmt::Debug + Default + Serialize + DeserializeOwned {
    type Error;
    // Useful only for non top level config?
    // Since top level config has so many usages.
    fn validate(&self) -> Result<(), Self::Error>;
    fn overwrite_from_env(&mut self);
}
