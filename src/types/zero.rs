use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt::{Display, Formatter};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone, FromPrimitive, ToPrimitive)]
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
pub enum Zero {
    Up = 0,
    Down = 1,
}

impl Display for Zero {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Up => "up",
                Self::Down => "down",
            }
        )
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl Zero {
    pub(crate) fn __str__(&self) -> String {
        self.to_string()
    }

    pub(crate) fn __repr__(&self) -> String {
        format!("Zero.{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use log::info;
    use test_log::test;

    #[test]
    fn test_display() {
        let cases = vec![(Zero::Up, "up"), (Zero::Down, "down")];

        for (zero, want) in cases {
            info!("Displaying {zero:?}, expecting {want:?}");
            let got = format!("{want}");
            assert_eq!(want, got);
        }
    }
}
