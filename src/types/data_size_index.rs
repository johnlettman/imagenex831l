use binrw::{BinRead, BinWrite};
use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt::{Display, Formatter};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

/// Size index of the sonar data field.
/// The IMAGENEX documentation refers to this as `nToReadIndex`.
#[derive(Debug, BinRead, BinWrite, Eq, PartialEq, Copy, Clone, ToPrimitive, FromPrimitive)]
#[repr(u8)]
#[brw(repr = u8)]
#[cfg_attr(
    target_family = "wasm",
    derive(tsify::Tsify, serde::Serialize, serde::Deserialize),
    tsify(into_wasm_abi, from_wasm_abi)
)]
#[cfg_attr(
    all(feature = "serde", not(target_family = "wasm")),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(feature = "pyo3", pyclass(eq))]
pub enum DataSizeIndex {
    #[cfg_attr(feature = "serde", serde(rename = "250_bytes"))]
    X250Bytes = 2,
}

impl DataSizeIndex {
    pub const fn bytes(&self) -> usize {
        match *self {
            Self::X250Bytes => 250,
        }
    }
}

impl Default for DataSizeIndex {
    fn default() -> Self {
        Self::X250Bytes
    }
}

impl Display for DataSizeIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::X250Bytes => "250 bytes",
            }
        )
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DataSizeIndex {
    pub(crate) fn __str__(&self) -> String {
        self.to_string()
    }

    pub(crate) fn __int__(&self) -> usize {
        self.bytes()
    }

    #[pyo3(name = "bytes")]
    pub(crate) fn py_bytes(&self) -> usize {
        self.bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use log::info;
    use test_log::test;

    #[test]
    fn test_bytes() {
        let cases = vec![(DataSizeIndex::X250Bytes, 250usize)];

        for (data_size_index, want) in cases {
            info!("Getting bytes for {data_size_index:?}, expecting {want}");
            let got = data_size_index.bytes();
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_default() {
        let want = DataSizeIndex::X250Bytes;
        let got = DataSizeIndex::default();
        assert_eq!(got, want, "it should default to {want:?}");
    }

    #[test]
    fn test_display() {
        let cases = vec![(DataSizeIndex::X250Bytes, "250 bytes")];

        for (data_size_index, want) in cases {
            info!("Displaying {data_size_index:?}, expecting {want:?}");
            let got = format!("{data_size_index}");
            assert_eq!(want, got);
        }
    }
}
