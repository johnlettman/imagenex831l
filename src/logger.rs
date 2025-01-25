#![allow(dead_code)]
#[cfg(not(feature = "pyo3"))]
use slog::{o, Drain, Duplicate, Logger};

#[cfg(not(feature = "pyo3"))]
use slog_async::Async;

#[cfg(feature = "pyo3")]
use once_cell::sync::Lazy;

#[cfg(feature = "pyo3")]
use std::sync::RwLock;

use crate::Result;

#[cfg(feature = "pyo3")]
static LOGGING_INITIALIZED: Lazy<RwLock<bool>> = Lazy::new(|| RwLock::new(false));

#[cfg(not(feature = "pyo3"))]
pub(crate) fn configure_logger() -> Result<()> {
    let _ = slog_scope::set_global_logger(get_root_logger()?);
    slog_stdlog::init()?;

    Ok(())
}

#[cfg(feature = "pyo3")]
pub(crate) fn configure_logger() -> Result<()> {
    if *LOGGING_INITIALIZED.read().unwrap() {
        Ok(())
    } else if pyo3_log::try_init().is_ok() {
        *LOGGING_INITIALIZED.write().unwrap() = true;
        Ok(())
    } else {
        Err(crate::Error::new("Failed to initialize logging".to_string()))
    }
}

#[cfg(not(feature = "pyo3"))]
pub(crate) fn get_root_logger() -> Result<Logger> {
    let drain = Duplicate(get_discard_drain()?, get_discard_drain()?).fuse();

    #[cfg(feature = "termlog")]
    let drain = Duplicate(get_termlog_drain().unwrap_or(get_discard_drain()?), drain).fuse();

    #[cfg(feature = "syslog")]
    let drain = Duplicate(get_syslog_drain().unwrap_or(get_discard_drain()?), drain).fuse();

    #[cfg(all(target_os = "linux", feature = "journald"))]
    let drain = Duplicate(get_journald_drain()?, drain).fuse();

    let logger = Logger::root(drain, o!("who" => "imagenex831l"));

    Ok(logger)
}

#[cfg(not(feature = "pyo3"))]
pub(crate) fn get_discard_drain() -> Result<Async> {
    let drain = Async::default(slog::Discard);
    Ok(drain)
}

#[cfg(all(feature = "termlog", not(feature = "pyo3")))]
pub(crate) fn get_termlog_drain() -> Result<Async> {
    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let termlog = slog_term::FullFormat::new(plain);
    let drain = Async::default(termlog.build().fuse());
    Ok(drain)
}

#[cfg(all(feature = "syslog", not(feature = "pyo3")))]
pub(crate) fn get_syslog_drain() -> Result<Async> {
    let syslog = slog_syslog::unix_3164(slog_syslog::Facility::LOG_USER)?;
    let drain = Async::default(syslog.fuse());
    Ok(drain)
}

#[cfg(all(target_os = "linux", feature = "journald", not(feature = "pyo3")))]
pub(crate) fn get_journald_drain() -> Result<Async> {
    let journald = slog_journald::JournaldDrain.ignore_res();
    let drain = Async::default(journald);
    Ok(drain)
}
