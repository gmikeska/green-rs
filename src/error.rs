//! Error handling for Green API
//!
//! This module provides a comprehensive error type that covers various failure modes
//! including IO errors, JSON parsing errors, CLI execution errors, and timeouts.

use thiserror::Error;

/// Result type alias for Green API operations
///
/// # Example
/// ```no_run
/// use green_rs::Result;
///
/// fn process_data() -> Result<String> {
///     Ok("Success".to_string())
/// }
/// ```
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for Green API
///
/// This enum covers all possible error cases that can occur during API operations.
#[derive(Error, Debug)]
pub enum Error {
    /// IO error from `std::io` operations
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// CLI error with non-zero exit code and stderr capture
    ///
    /// This variant should contain the stderr output when a CLI command fails
    #[error("CLI error: {0}")]
    Cli(String),

    /// Timeout error for operations that exceed time limits
    #[error("Operation timed out")]
    Timeout,

    /// Network-related errors
    #[error("Network error: {0}")]
    Network(String),

    /// Invalid response received from the API
    #[error("Invalid response received")]
    InvalidResponse,

    /// Catch-all for unexpected errors
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

impl Error {
    /// Create a new CLI error with the given stderr output
    ///
    /// # Example
    /// ```no_run
    /// use green_rs::Error;
    ///
    /// let error = Error::cli_error("Command failed: permission denied");
    /// ```
    pub fn cli_error<S: Into<String>>(stderr: S) -> Self {
        Self::Cli(stderr.into())
    }

    /// Create a new network error
    pub fn network<S: Into<String>>(msg: S) -> Self {
        Self::Network(msg.into())
    }

    /// Create a new unexpected error
    pub fn unexpected<S: Into<String>>(msg: S) -> Self {
        Self::Unexpected(msg.into())
    }
}
