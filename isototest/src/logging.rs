//! This module provides a sensible default configuration of a logging system.

#[cfg(feature = "default-logging")]
use crate::errors::util_errors::LoggingError;
#[cfg(feature = "default-logging")]
use env_logger::Builder;
#[cfg(feature = "default-logging")]
use std::io::Write;

pub const LOG_TARGET: &str = "[isototest]";

#[cfg(feature = "default-logging")]
/// Initialize default logging configuration with given log level.
///
/// Log Level can be:
/// * "info" - Default log level also adapted if level is `none`. Logs events relevant for general usage.
/// * "debug" - Debug level for extended logging. Should only be used for development purposes.
/// * "trace" - Extensive log evel logging every event. Should only be used for development purposes.
///
pub(crate) fn init_default_logging(level: Option<&str>) -> Result<(), LoggingError> {
    match level {
        Some("info") | None => {
            log_builder(log::LevelFilter::Info);
            Ok(())
        }
        Some("debug") => {
            log_builder(log::LevelFilter::Debug);
            Ok(())
        }
        Some("trace") => {
            log_builder(log::LevelFilter::Trace);
            Ok(())
        }
        Some(invalid) => Err(LoggingError::InvalidLogLevelError(format!(
            "Invalid log level '{}'!",
            invalid
        ))),
    }
}

#[cfg(feature = "default-logging")]
fn log_builder(level: log::LevelFilter) {
    Builder::new()
        .filter_level(level)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {}: {}",
                buf.timestamp(),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();
}
