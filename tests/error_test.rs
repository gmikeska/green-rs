//! Tests for the error module

use green_rs::{Error, Result};
use std::io;

#[test]
fn test_io_error_conversion() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "File not found");
    let error: Error = io_err.into();

    match error {
        Error::Io(_) => (),
        _ => panic!("Expected Io error variant"),
    }
}

#[test]
fn test_json_error_conversion() {
    let json_str = r#"{"invalid": json}"#;
    let result: std::result::Result<serde_json::Value, Error> =
        serde_json::from_str(json_str).map_err(Into::into);

    assert!(result.is_err());
    match result.unwrap_err() {
        Error::Json(_) => (),
        _ => panic!("Expected Json error variant"),
    }
}

#[test]
fn test_cli_error() {
    let error = Error::cli_error("Command failed with exit code 1: permission denied");

    match error {
        Error::Cli(msg) => {
            assert!(msg.contains("permission denied"));
        }
        _ => panic!("Expected Cli error variant"),
    }
}

#[test]
fn test_timeout_error() {
    let error = Error::Timeout;
    assert_eq!(error.to_string(), "Operation timed out");
}

#[test]
fn test_result_type_alias() {
    fn returns_result() -> Result<String> {
        Ok("Success".to_string())
    }

    let result = returns_result();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Success");
}

#[test]
fn test_error_display() {
    let io_err = Error::Io(io::Error::new(io::ErrorKind::NotFound, "test.txt"));
    assert!(io_err.to_string().contains("IO error"));

    let cli_err = Error::cli_error("stderr output");
    assert_eq!(cli_err.to_string(), "CLI error: stderr output");

    let timeout_err = Error::Timeout;
    assert_eq!(timeout_err.to_string(), "Operation timed out");
}
