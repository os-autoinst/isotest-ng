//! Prepare a [openQA](https://open.qa) test worker.
//!
//! This crate is responsible for setting up a `openQA` worker machine or VM.
//!
//! ## Optional Features
//!
//! * `default-logging` - Provides you with a sensible logger configuration using the `env_logger` crate.

// Organize library structure
pub mod logging;
pub mod errors;

// Provide code on the root level of the library
#[cfg(feature = "default-logging")]
use crate::logging::init_default_logging;
#[cfg(feature = "default-logging")]
use crate::errors::util_errors::LoggingError;

#[cfg(feature = "default-logging")]
pub fn init_logging(level: Option<&str>) -> Result<(), LoggingError> {
    init_default_logging(level)?;
    Ok(())
}
