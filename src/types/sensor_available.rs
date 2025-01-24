use binrw::{BinRead, BinWrite};
use std::fmt::{Display, Formatter};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[derive(Debug, BinRead, BinWrite, Eq, PartialEq, Copy, Clone)]
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
pub enum SensorAvailable {
    NotAvailable = 0,
    Available = 1,
}

impl Default for SensorAvailable {
    fn default() -> Self {
        Self::NotAvailable
    }
}

impl Display for SensorAvailable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                SensorAvailable::NotAvailable =>
                    "no external pitch / roll / distance sensor present",
                SensorAvailable::Available => "external pitch / roll / distance sensor present",
            }
        )
    }
}

impl From<bool> for SensorAvailable {
    fn from(value: bool) -> Self {
        match value {
            true => SensorAvailable::Available,
            false => Self::NotAvailable,
        }
    }
}

impl Into<bool> for SensorAvailable {
    fn into(self) -> bool {
        self == SensorAvailable::Available
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl SensorAvailable {
    #[new]
    pub(crate) fn py_new(available: bool) -> Self {
        available.into()
    }

    pub(crate) fn __str__(&self) -> String {
        self.to_string()
    }

    pub(crate) fn __bool__(&self) -> bool {
        (*self).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let got = SensorAvailable::default();
        assert_eq!(got, SensorAvailable::NotAvailable);
    }

    #[test]
    fn test_display() {
        let cases = vec![
            (SensorAvailable::NotAvailable, "no external pitch / roll / distance sensor present"),
            (SensorAvailable::Available, "external pitch / roll / distance sensor present"),
        ];

        for (sensor_available, want) in cases {
            let got = format!("{sensor_available}");
            assert_eq!(got, want);
        }
    }
}
