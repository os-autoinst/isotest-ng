//! This module defines and implements error type which refer to the supporting util of this library.
//! This means functionality like logging and similar supporting functionality, which do not necessarily
//! impact the core functionality of this library itself.
use std::fmt;

#[derive(Debug)]
#[cfg(feature = "default-logging")]
pub enum LoggingError {
    LoggingInitError(String),
    InvalidLogLevelError(String),
}

#[cfg(feature = "default-logging")]
impl fmt::Display for LoggingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoggingError::LoggingInitError(msg) => {
                write!(f, "[error] Logging initialization failed: '{}'", msg)
            }
            LoggingError::InvalidLogLevelError(msg) => {
                write!(f, "[error] Invalid log level: '{}'", msg)
            }
        }
    }
}

#[cfg(feature = "default-logging")]
impl std::error::Error for LoggingError {}
