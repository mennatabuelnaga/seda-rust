#[macro_export]
macro_rules! env_overwrite {
    ($field:expr, $name:expr) => {
        if let Some(var) = std::env::var($name).ok() {
            $field = var.into();
        }
    };
}


#[macro_export]
macro_rules! overwrite_config_field {
    ($field:expr, $value:expr) => {
        if let Some(var) = $value {
            $field.replace_range(.., &var);
        }
    };
}
