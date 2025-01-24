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
pub enum Direction {
    Counterclockwise = 0,
    Clockwise = 1,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Counterclockwise
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Clockwise => "clockwise",
                Self::Counterclockwise => "counterclockwise",
            }
        )
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl Direction {
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
    fn test_default() {
        let want = Direction::Counterclockwise;
        let got = Direction::default();
        assert_eq!(want, got);
    }

    #[test]
    fn test_display() {
        let cases = vec![
            (Direction::Clockwise, "clockwise"),
            (Direction::Counterclockwise, "counterclockwise"),
        ];

        for (direction, want) in cases {
            info!("Displaying {direction:?}, expecting {want:?}");
            let got = format!("{direction}");
            assert_eq!(want, got);
        }
    }
}
