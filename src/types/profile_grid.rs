use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt::{Display, Formatter};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone, FromPrimitive, ToPrimitive)]
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
pub enum ProfileGrid {
    Off = 0,
    On = 1,
}

impl Default for ProfileGrid {
    fn default() -> Self {
        Self::Off
    }
}

impl Display for ProfileGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Off => "off",
                Self::On => "on",
            }
        )
    }
}

impl From<bool> for ProfileGrid {
    fn from(value: bool) -> Self {
        match value {
            true => Self::On,
            false => Self::Off,
        }
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl ProfileGrid {
    #[new]
    pub(crate) fn py_new(enable: bool) -> Self {
        enable.into()
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

    use log::info;
    use test_log::test;

    #[test]
    fn default() {
        let want = ProfileGrid::Off;
        let got = ProfileGrid::default();
        assert_eq!(want, got);
    }

    #[test]
    fn display() {
        let cases = vec![(ProfileGrid::Off, "off"), (ProfileGrid::On, "on")];

        for (profile_grid, want) in cases {
            info!("Displaying {profile_grid:?}, want {want}");
            let got = format!("{profile_grid}");
            assert_eq!(want, got);
        }
    }

    #[test]
    fn from_bool() {
        assert_eq!(ProfileGrid::from(true), ProfileGrid::On);
        assert_eq!(ProfileGrid::from(false), ProfileGrid::Off);
    }
}
