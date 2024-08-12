mod array;
mod build;
mod doc;
mod file;
mod shot;
mod sonar_return;
mod switch_data;

pub mod types;

pub use shot::Shot;
pub use sonar_return::SonarReturn;
pub use switch_data::SwitchData;

pub use array::Array;
pub use file::File;

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("UTF-8 encoding error")]
    Utf8Encoding(#[from] std::str::Utf8Error),

    #[error("date time parsing error")]
    DateTimeParse(#[from] chrono::ParseError),

    #[error("binary encoding error")]
    BinaryEncoding(#[from] binrw::Error),

    #[error("I/O error")]
    IO(#[from] std::io::Error),
}

pub type Result<T> = core::result::Result<T, Error>;
