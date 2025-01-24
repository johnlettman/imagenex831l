use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt::{Display, Formatter};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone, ToPrimitive, FromPrimitive)]
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
pub enum StepDirection {
    Normal = 0,
    Reverse = 1,
}

impl Default for StepDirection {
    fn default() -> Self {
        Self::Normal
    }
}

impl Display for StepDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Normal => "normal",
                Self::Reverse => "reverse step direction",
            }
        )
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl StepDirection {
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
    fn default() {
        let got = StepDirection::default();
        assert_eq!(StepDirection::Normal, got);
    }

    #[test]
    fn display() {
        let cases = vec![
            (StepDirection::Normal, "normal"),
            (StepDirection::Reverse, "reverse step direction"),
        ];

        for (reverse, want) in cases {
            info!("Displaying {reverse:?}, expecting {want:?}");
            let got = format!("{reverse}");
            assert_eq!(want, got);
        }
    }
}
