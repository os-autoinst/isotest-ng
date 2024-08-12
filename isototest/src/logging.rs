//! This module provides a sensible default configuration of a logging system.

#[cfg(feature = "default-logging")]
use env_logger::Builder;
#[cfg(feature = "default-logging")]
use std::io::Write;

pub const LOG_TARGET: &str = "[isototest]";

#[cfg(feature = "default-logging")]
/// Initialize default logging configuration.
///
/// By default, we will only log `info` leves and higher.
pub fn init_default_logging() {
    Builder::new()
        .filter_level(log::LevelFilter::Info)
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

#[cfg(feature = "default-logging")]
/// Initialize debug logging.
///
/// Will log every event with the `debug` log level and higher.
///
/// **Should only be used for development purposes.**
pub fn init_debug_logging() {
    Builder::new()
        .filter_level(log::LevelFilter::Debug)
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

#[cfg(feature = "default-logging")]
/// Initialize trace.
///
/// Will log every event.
///
/// **Should only be used for development purposes.**
pub fn init_trace() {
    Builder::new()
        .filter_level(log::LevelFilter::Trace)
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
