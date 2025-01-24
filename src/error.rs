use std::fmt;

#[derive(thiserror::Error, Debug)]
pub struct Error {
    pub message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    #[cfg(feature = "nightly")]
    backtrace: std::backtrace::Backtrace,
}

impl Error {
    pub fn new(message: String) -> Self {
        Self {
            message,
            source: None,
            #[cfg(feature = "nightly")]
            backtrace: std::backtrace::Backtrace::capture(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Default for Error {
    fn default() -> Self {
        Error {
            message: "".to_string(),
            source: None,
            #[cfg(feature = "nightly")]
            backtrace: std::backtrace::Backtrace::capture(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error {
            message: String::from("IO Error"),
            source: Some(Box::new(error)),
            #[cfg(feature = "nightly")]
            backtrace: std::backtrace::Backtrace::capture(),
        }
    }
}

impl From<log::SetLoggerError> for Error {
    fn from(error: log::SetLoggerError) -> Self {
        Error {
            message: String::from("Logging Error"),
            source: Some(Box::new(error)),
            #[cfg(feature = "nightly")]
            backtrace: std::backtrace::Backtrace::capture(),
        }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        Error {
            message: String::from("UTF-8 Encoding Error"),
            source: Some(Box::new(error)),
            #[cfg(feature = "nightly")]
            backtrace: std::backtrace::Backtrace::capture(),
        }
    }
}

impl From<chrono::ParseError> for Error {
    fn from(error: chrono::ParseError) -> Self {
        Error {
            message: String::from("Date/Time Parse Error"),
            source: Some(Box::new(error)),
            #[cfg(feature = "nightly")]
            backtrace: std::backtrace::Backtrace::capture(),
        }
    }
}

impl From<binrw::Error> for Error {
    fn from(error: binrw::Error) -> Self {
        Error {
            message: String::from("Binary Encoding Error"),
            source: Some(Box::new(error)),
            #[cfg(feature = "nightly")]
            backtrace: std::backtrace::Backtrace::capture(),
        }
    }
}

#[cfg(feature = "pyo3")]
impl From<pyo3::PyErr> for Error {
    fn from(error: pyo3::PyErr) -> Self {
        Error {
            message: error.to_string(),
            source: Some(Box::new(error)),
            #[cfg(feature = "nightly")]
            backtrace: std::backtrace::Backtrace::capture(),
        }
    }
}

#[cfg(feature = "pyo3")]
impl Into<pyo3::PyErr> for Error {
    fn into(self) -> pyo3::PyErr {
        use pyo3::exceptions::*;

        // Match specific error sources to Python exceptions
        if let Some(source) = self.source {
            if let Some(io_error) = source.downcast_ref::<std::io::Error>() {
                return PyIOError::new_err(format!("IO Error: {}", io_error));
            }
            if let Some(utf8_error) = source.downcast_ref::<std::str::Utf8Error>() {
                return PyValueError::new_err(format!("UTF-8 Encoding Error: {}", utf8_error));
            }
            if let Some(parse_error) = source.downcast_ref::<chrono::ParseError>() {
                return PyValueError::new_err(format!("Date/Time Parse Error: {}", parse_error));
            }
            if let Some(binrw_error) = source.downcast_ref::<binrw::Error>() {
                return PyRuntimeError::new_err(format!("Binary Encoding Error: {}", binrw_error));
            }
            if let Some(logger_error) = source.downcast_ref::<log::SetLoggerError>() {
                return PyRuntimeError::new_err(format!("Logging Error: {}", logger_error));
            }
        }

        // Fallback to a generic Python exception
        PyRuntimeError::new_err(self.message)
    }
}
