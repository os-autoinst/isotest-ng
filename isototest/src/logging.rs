//! This module provides a sensible default configuration of a logging system.

#[cfg(feature = "default-logging")]
use env_logger::Builder;
#[cfg(feature = "default-logging")]
use std::io::Write;

pub const LOG_TARGET: &str = "[isototest]";

#[cfg(feature = "default-logging")]
/// Initialize default logging configuration.
pub fn initialize_default_logging() {
    Builder::new()
        .filter_level(log::LevelFilter::Info)
        .format(|bug, record| {
            writeln!(
                buf,
                "{} [{}] {}: {}",
                buf.timestamp,
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();
}
