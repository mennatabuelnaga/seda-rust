use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("environment variable `{0}` is not set")]
    MissingEnvVar(String),
}

pub fn get_env_var(var_name: &str) -> Result<String, CliError> {
    match std::env::var(var_name) {
        Ok(val) => Ok(val),
        Err(_) => Err(CliError::MissingEnvVar(var_name.to_string())),
    }
}
