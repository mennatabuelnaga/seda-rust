use serde::{de::DeserializeOwned, Serialize};

#[macro_export]
macro_rules! overwrite_config_field {
    ($field:expr, $value:expr) => {
        if let Some(var) = $value {
            $field.replace(var);
        }
    };
}

#[macro_export]
macro_rules! env_overwrite {
    ($field:expr, $name:expr) => {
        seda_config::overwrite_config_field!($field, std::env::var($name).ok());
    };
}

pub trait Config: std::fmt::Debug + Default + Serialize + DeserializeOwned {
    fn template() -> Self;
    fn overwrite_from_env(&mut self);
}
