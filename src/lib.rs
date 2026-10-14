//! Purpose: Expose the persistent design tracer's typed protocol.
//! Responsibilities: share model, evaluation, storage and rendering across callers.
//! Rationale: funkalization-0ps uses one semantic path for CLI and direct API tests.

pub mod model;
pub mod store;
pub mod tracer;
pub mod view;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Error {
    pub code: String,
    pub message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}
impl std::error::Error for Error {}
pub type Result<T> = std::result::Result<T, Error>;
pub fn error(code: &str, message: impl ToString) -> Error {
    Error {
        code: code.into(),
        message: message.to_string(),
    }
}
