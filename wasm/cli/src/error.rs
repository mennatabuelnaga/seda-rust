use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {}

pub type Result<T, E = CliError> = core::result::Result<T, E>;