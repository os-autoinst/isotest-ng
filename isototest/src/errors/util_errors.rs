//! This module defines and implements error type which refer to the supporting util of this library.
//! This means functionality like logging and similar supporting functionality, which do not necessarily
//! impact the core functionality of this library itself.
use std::fmt;

#[derive(Debug)]
#[cfg(feature = "default-logging")]
/// Error used for checking given log level.
/// Only available as part of the `default-logging` feature.
pub struct InvalidLogLevelError(pub String);

#[cfg(feature = "default-logging")]
impl fmt::Display for InvalidLogLevelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[error] Invalid log level: '{}'", self.0)
    }
}

#[cfg(feature = "default-logging")]
impl std::error::Error for InvalidLogLevelError {}
