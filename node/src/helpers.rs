pub type Result<T, E = CliError> = core::result::Result<T, E>;

pub fn get_env_var(var_name: &str) -> Result<String, CliError> {
    match std::env::var(var_name) {
        Ok(val) => Ok(val),
        Err(_) => Err(CliError::MissingEnvVar(var_name.to_string())),
    }
}
