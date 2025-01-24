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
pub enum Transducer {
    Down = 0,
    Up = 1,
}

impl Display for Transducer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Down => "down",
                Self::Up => "up",
            }
        )
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl Transducer {
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
    fn test_display() {
        let cases = vec![(Transducer::Down, "down"), (Transducer::Up, "up")];

        for (transducer, want) in cases {
            info!("Displaying {transducer:?}, expecting {want:?}");
            let got = format!("{transducer}");
            assert_eq!(want, got);
        }
    }
}
