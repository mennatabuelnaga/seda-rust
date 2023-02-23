use serde::{Deserialize, Serialize};

/// Represents the result of a Vm instance
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct VmResult {
    pub stdout:    Vec<String>,
    pub stderr:    Vec<String>,
    pub result:    Option<Vec<u8>>,
    pub exit_code: u8,
}

/// The possible statuses of a [VmResult]
#[derive(Debug)]
pub enum VmResultStatus {
    /// When the Vm has nothing in the promise queue to run
    EmptyQueue,
    /// When the Vm runs and exits successfully
    OK,
    FailedToSetConfig,
    WasiEnvInitializeFailure,
    FailedToCreateVMImports,
    FailedToCreateWasmerInstance,
    FailedToGetWASMFn,
    FailedToGetWASMStdout,
    FailedToGetWASMStderr,
    ExecutionError(String),
}

macro_rules! vm_result_status {
    ($stdout:expr, $stderr:expr, $res:expr, $exit_code:expr) => {
        (
            vec![$stdout.to_string()],
            vec![$stderr.to_string()],
            Some($res),
            $exit_code,
        )
    };
    ($stdout:expr, $stderr:expr, $exit_code:expr) => {
        (vec![$stdout.to_string()], vec![$stderr.to_string()], None, $exit_code)
    };
    (@o $stdout:expr, $res:expr, $exit_code:expr) => {
        (vec![$stdout.to_string()], Vec::new(), Some($res), $exit_code)
    };
    (@o $stdout:expr, $exit_code:expr) => {
        (vec![$stdout.to_string()], Vec::new(), None, $exit_code)
    };
    (@e $stderr:expr, $res:expr, $exit_code:expr) => {
        (Vec::new(), vec![$stderr.to_string()], Some($res), $exit_code)
    };
    (@e $stderr:expr, $exit_code:expr) => {
        (Vec::new(), vec![$stderr.to_string()], None, $exit_code)
    };
}

impl From<VmResultStatus> for VmResult {
    fn from(value: VmResultStatus) -> Self {
        let (stdout, stderr, result, exit_code) = match value {
            VmResultStatus::EmptyQueue => vm_result_status!(@o "Success(Empty Promise Queue)", 0),
            VmResultStatus::OK => vm_result_status!(@o "Success", 0),
            VmResultStatus::FailedToSetConfig => vm_result_status!(@e "Failed to set VM Config", 1),
            VmResultStatus::WasiEnvInitializeFailure => vm_result_status!(@e "Failed to initialize Wasi Env", 2),
            _ => todo!(),
        };

        VmResult {
            stdout,
            stderr,
            result,
            exit_code,
        }
    }
}

impl From<VmResultStatus> for ExecutionResult {
    fn from(value: VmResultStatus) -> Self {
        Ok(value)
    }
}

pub type ExecutionResult<T = VmResultStatus, E = VmResultStatus> = core::result::Result<T, E>;

impl From<ExecutionResult> for VmResult {
    fn from(value: ExecutionResult) -> Self {
        match value {
            Ok(ok) => ok.into(),
            Err(err) => err.into(),
        }
    }
}
