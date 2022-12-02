#[macro_export]
macro_rules! overwrite_config_field {
    ($field:expr, $value:expr) => {
        if let Some(var) = $value {
            $field.replace(var.into());
        }
    };
}

#[macro_export]
macro_rules! env_overwrite {
    ($field:expr, $name:expr) => {
        $crate::overwrite_config_field!($field, std::env::var($name).ok());
    };
}
