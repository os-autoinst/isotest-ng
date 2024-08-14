//! Prepare a test environment for [openQA](https://open.qa).
//!
//! This crate is responsible for preparing an `openQA` worker machine or VM for executing the test suite.
//!
//! ## Optional Features
//!
//! * `default-logging` - Provides you with a sensible logger configuration using the `env_logger` crate.

// Organize library structure
pub mod errors;
pub mod logging;

// Provide code on the root level of the library
#[cfg(feature = "default-logging")]
use crate::errors::util_errors::LoggingError;
#[cfg(feature = "default-logging")]
use crate::logging::init_default_logging;

#[cfg(feature = "default-logging")]
pub fn init_logging(level: Option<&str>) -> Result<(), LoggingError> {
    init_default_logging(level)?;
    Ok(())
}
