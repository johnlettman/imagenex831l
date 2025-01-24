use binrw::{BinRead, BinWrite};
use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt::{Display, Formatter};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone, BinRead, BinWrite, FromPrimitive, ToPrimitive)]
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
#[cfg_attr(feature = "pyo3", pyclass(eq, eq_int))]
pub enum Mode {
    Sector = 0,
    Polar = 1,
    Sidescan = 2,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Sector => "sector",
                Self::Polar => "polar",
                Self::Sidescan => "sidescan",
            }
        )
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl Mode {
    pub(crate) fn __str__(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let cases =
            vec![(Mode::Sector, "sector"), (Mode::Polar, "polar"), (Mode::Sidescan, "sidescan")];

        for (mode, want) in cases {
            let got = format!("{mode}");
            assert_eq!(want, got);
        }
    }
}
