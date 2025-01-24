use num_derive::{FromPrimitive, ToPrimitive};
use std::cmp::Ordering;
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
#[cfg_attr(feature = "pyo3", pyclass(eq, eq_int, ord))]
pub enum DataBits {
    #[cfg_attr(feature = "serde", serde(rename = "4_bits"))]
    X4Bits = 0,

    #[cfg_attr(feature = "serde", serde(rename = "8_bits"))]
    X8Bits = 1,

    #[cfg_attr(feature = "serde", serde(rename = "14_bits"))]
    X14Bits = 2,
}

impl DataBits {
    pub fn bits(self) -> u8 {
        match self {
            Self::X4Bits => 4,
            Self::X8Bits => 8,
            Self::X14Bits => 14,
        }
    }
}

impl Display for DataBits {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{} bits", self.bits())
    }
}

impl Ord for DataBits {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bits().cmp(&other.bits())
    }
}

impl PartialOrd<Self> for DataBits {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DataBits {
    pub(crate) fn __int__(&self) -> u8 {
        self.bits()
    }

    pub(crate) fn __str__(&self) -> String {
        self.to_string()
    }

    #[pyo3(name = "bits")]
    pub(crate) fn py_bits(&self) -> u8 {
        self.bits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use log::info;
    use test_log::test;

    #[test]
    fn bits() {
        let cases = vec![(DataBits::X4Bits, 4), (DataBits::X8Bits, 8), (DataBits::X14Bits, 14)];

        for (data_bits, want) in cases {
            info!("Getting bits from {data_bits:?}, expecting {want:?}");
            let got = data_bits.bits();
            assert_eq!(want, got);
        }
    }

    #[test]
    fn display() {
        let cases = vec![
            (DataBits::X4Bits, "4 bits"),
            (DataBits::X8Bits, "8 bits"),
            (DataBits::X14Bits, "14 bits"),
        ];

        for (data_bits, want) in cases {
            info!("Displaying {data_bits:?}, expecting {want:?}");
            let got = format!("{data_bits}");
            assert_eq!(want, got);
        }
    }

    #[test]
    fn ord() {
        let cases = vec![
            (DataBits::X4Bits, DataBits::X8Bits, Ordering::Less),
            (DataBits::X4Bits, DataBits::X4Bits, Ordering::Equal),
            (DataBits::X14Bits, DataBits::X4Bits, Ordering::Greater),
        ];

        for (data_bits_1, data_bits_2, want) in cases {
            info!("Ordering {data_bits_1:?} against {data_bits_2:?}, want {want:?}");
            let got = data_bits_1.cmp(&data_bits_2);
            assert_eq!(want, got);

            let got = data_bits_1.partial_cmp(&data_bits_2);
            assert!(got.is_some());
            assert_eq!(want, got.unwrap());
        }
    }
}
