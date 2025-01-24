use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt::{Display, Formatter};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

/// New in Ethernet Specification v1.01, revision `03` from March 22, 2023.
#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(u8)]
#[cfg_attr(
    target_family = "wasm",
    derive(tsify::Tsify, serde::Serialize, serde::Deserialize),
    tsify(into_wasm_abi, from_wasm_abi)
)]
#[cfg_attr(
    all(feature = "serde", not(target_family = "wasm")),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "pyo3", pyclass(eq))]
pub enum ProfilePointDetection {
    CenterOfPulse = 0,
    StartOfPulse = 1,
}

impl Display for ProfilePointDetection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::CenterOfPulse => "center of pulse",
                Self::StartOfPulse => "start of pulse",
            }
        )
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl ProfilePointDetection {
    pub(crate) fn __str__(&self) -> String {
        self.to_string()
    }
}
