#[macro_export]
macro_rules! env_overwrite {
    ($field:expr, $name:expr) => {
        if let Some(var) = std::env::var($name).ok() {
            $field = var.into();
        }
    };
}
