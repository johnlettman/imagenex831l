//! # imagenex831l
//!
//! `imagenex831l` is a library for interacting with IMAGENEX 831L sonar units and their data.

mod build;
mod doc;
mod shot;
mod sonar_return;
mod switch_data;

mod error;
mod io;
pub(crate) mod logger;
pub mod types;

pub use io::Reader;
pub use shot::Shot;
pub use sonar_return::SonarReturn;
pub use switch_data::SwitchData;

pub use build::{IDENTIFIER, VERSION};
pub use error::Error;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg(feature = "pyo3")]
#[pymodule(name = "imagenex831l")]
fn py_init(module: &Bound<'_, PyModule>) -> PyResult<()> {
    logger::configure_logger().map_err(|e| Into::<PyErr>::into(e))?;
    log::debug!("Initialized logger");

    module.add("__version__", VERSION)?;
    module.add("__identifier__", IDENTIFIER)?;

    module.add_wrapped(pyo3::wrap_pymodule!(types::types))?;
    module.add_class::<Reader>()?;
    Ok(())
}

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub type Result<T> = core::result::Result<T, Error>;
