pub fn get_env_var(var_name: &str) -> crate::errors::Result<String> {
    match std::env::var(var_name) {
        Ok(val) => Ok(val),
        Err(_) => Err(crate::errors::CliError::MissingEnvVar(var_name.to_string())),
    }
}
