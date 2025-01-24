use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone, ToPrimitive, FromPrimitive)]
#[cfg_attr(
    target_family = "wasm",
    derive(tsify::Tsify, serde::Serialize, serde::Deserialize),
    tsify(into_wasm_abi, from_wasm_abi)
)]
#[cfg_attr(
    all(feature = "serde", not(target_family = "wasm")),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(feature = "pyo3", pyclass(eq, eq_int, ord))]
pub enum Logf {
    #[cfg_attr(feature = "serde", serde(rename = "10dB"))]
    X10dB = 0,

    #[cfg_attr(feature = "serde", serde(rename = "20dB"))]
    X20dB = 1,

    #[cfg_attr(feature = "serde", serde(rename = "30dB"))]
    X30dB = 2,

    #[cfg_attr(feature = "serde", serde(rename = "40dB"))]
    X40dB = 3,
}

impl Logf {
    pub const fn decibels(&self) -> usize {
        match *self {
            Logf::X10dB => 10,
            Logf::X20dB => 20,
            Logf::X30dB => 30,
            Logf::X40dB => 40,
        }
    }
}

impl Display for Logf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} dB", self.decibels())
    }
}

impl Ord for Logf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_u8().cmp(&other.to_u8())
    }
}

impl PartialOrd for Logf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl Logf {
    pub(crate) fn __str__(&self) -> String {
        self.to_string()
    }

    pub(crate) fn __int__(&self) -> usize {
        self.decibels()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use log::info;
    use test_log::test;

    #[test]
    fn test_decibels() {
        let cases = vec![
            (Logf::X10dB, 10usize),
            (Logf::X20dB, 20usize),
            (Logf::X30dB, 30usize),
            (Logf::X40dB, 40usize),
        ];

        for (logf, want) in cases {
            info!("Getting decibels for {logf:?}, expecting {want}");
            let got = logf.decibels();
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_display() {
        let cases = vec![
            (Logf::X10dB, "10 dB"),
            (Logf::X20dB, "20 dB"),
            (Logf::X30dB, "30 dB"),
            (Logf::X40dB, "40 dB"),
        ];

        for (logf, want) in cases {
            info!("Displaying {logf:?}, expecting {want:?}");
            let got = format!("{logf}");
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_ord() {
        let cases = vec![
            (Logf::X10dB, Logf::X10dB, Ordering::Equal),
            (Logf::X10dB, Logf::X30dB, Ordering::Less),
            (Logf::X40dB, Logf::X20dB, Ordering::Greater),
        ];

        for (a, b, want) in cases {
            info!("Comparing {a:?} and {b:?}, expecting {want:?}");

            let got = a.partial_cmp(&b).expect("Should not be None");
            assert_eq!(want, got);

            let got = a.cmp(&b);
            assert_eq!(want, got);
        }
    }
}
