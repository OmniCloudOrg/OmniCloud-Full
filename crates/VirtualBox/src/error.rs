//! Custom error types for VirtualBox plugin

use std::fmt;

#[derive(Debug, Clone)]
pub enum VBoxError {
    ExecutionFailed(String),
    MissingArgument(String),
    InvalidArgument(String),
    NotInstalled(String),
    OperationFailed(String),
    ParseError(String),
}

impl fmt::Display for VBoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VBoxError::ExecutionFailed(msg) => write!(f, "VBoxManage execution failed: {}", msg),
            VBoxError::MissingArgument(arg) => write!(f, "Missing required argument: {}", arg),
            VBoxError::InvalidArgument(arg) => write!(f, "Invalid argument value: {}", arg),
            VBoxError::NotInstalled(msg) => write!(f, "VirtualBox not found or not working: {}", msg),
            VBoxError::OperationFailed(msg) => write!(f, "VM operation failed: {}", msg),
            VBoxError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for VBoxError {}

impl From<VBoxError> for String {
    fn from(error: VBoxError) -> Self {
        error.to_string()
    }
}

// Convenience result type
pub type VBoxResult<T> = Result<T, VBoxError>;