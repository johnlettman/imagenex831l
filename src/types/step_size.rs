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
pub enum StepSize {
    Slow = 0,
    Medium = 1,
    Fast = 2,
    Faster = 3,
    Fastest = 4,
}

impl StepSize {
    pub const fn degrees(&self) -> f32 {
        match *self {
            Self::Slow => 0.3,
            Self::Medium => 0.6,
            Self::Fast => 0.9,
            Self::Faster => 1.2,
            Self::Fastest => 2.4,
        }
    }
}

impl Display for StepSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Slow => "slow",
                Self::Medium => "medium",
                Self::Fast => "fast",
                Self::Faster => "faster",
                Self::Fastest => "fastest",
            }
        )
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl StepSize {
    pub(crate) fn __str__(&self) -> String {
        self.to_string()
    }

    pub(crate) fn py_degrees(&self) -> f32 {
        self.degrees()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use log::info;
    use test_log::test;

    #[test]
    fn test_degrees() {
        let cases = vec![
            (StepSize::Slow, 0.3),
            (StepSize::Medium, 0.6),
            (StepSize::Fast, 0.9),
            (StepSize::Faster, 1.2),
            (StepSize::Fastest, 2.4),
        ];

        for (step_size, want) in cases {
            info!("Getting degrees for {step_size:?}, expecting {want}");
            let got = step_size.degrees();
            assert_eq!(got, want);
        }
    }

    #[test]
    fn test_display() {
        let cases = vec![
            (StepSize::Slow, "slow"),
            (StepSize::Medium, "medium"),
            (StepSize::Fast, "fast"),
            (StepSize::Faster, "faster"),
            (StepSize::Fastest, "fastest"),
        ];

        for (step_size, want) in cases {
            info!("Displaying {step_size:?}, expecting {want:?}");
            let got = format!("{step_size}");
            assert_eq!(got, want);
        }
    }
}
