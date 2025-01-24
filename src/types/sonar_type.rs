use binrw::{BinRead, BinWrite};
use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt::{Display, Formatter};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone, BinRead, BinWrite, ToPrimitive, FromPrimitive)]
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
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "pyo3", pyclass(eq))]
pub enum SonarType {
    Scanning = 0,
    FixedPosition = 1,
}

impl Display for SonarType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Scanning => "scanning",
                Self::FixedPosition => "fixed position",
            }
        )
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl SonarType {
    pub(crate) fn __str__(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use log::info;
    use test_log::test;

    #[test]
    fn display() {
        let cases =
            vec![(SonarType::Scanning, "scanning"), (SonarType::FixedPosition, "fixed position")];

        for (sonar_type, want) in cases {
            info!("Displaying {sonar_type:?}, expecting {want:?}");
            let got = format!("{sonar_type}");
            assert_eq!(want, got);
        }
    }
}
