use binrw::{BinRead, BinWrite};
use num_derive::{FromPrimitive, ToPrimitive};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

/// The number of points in the sonar data field.
#[derive(Debug, BinRead, BinWrite, Eq, PartialEq, Copy, Clone, ToPrimitive, FromPrimitive)]
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
#[cfg_attr(feature = "pyo3", pyclass(eq))]
pub enum DataPoints {
    /// 250 data points will be returned by the head.
    /// The data will contain the `IMX` header.
    #[cfg_attr(feature = "serde", serde(rename = "250_points"))]
    X250Points = 2,
}

impl DataPoints {
    pub const fn points(&self) -> usize {
        match *self {
            Self::X250Points => 250,
        }
    }
}

impl Default for DataPoints {
    fn default() -> Self {
        Self::X250Points
    }
}

impl Display for DataPoints {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} points", self.points())
    }
}

impl Ord for DataPoints {
    fn cmp(&self, other: &Self) -> Ordering {
        self.points().cmp(&other.points())
    }
}

impl PartialOrd for DataPoints {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.points().cmp(&other.points()))
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DataPoints {
    pub(crate) fn __str__(&self) -> String {
        self.to_string()
    }

    pub(crate) fn __int__(&self) -> usize {
        self.points()
    }

    #[pyo3(name = "points")]
    pub(crate) fn py_points(&self) -> usize {
        self.points()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use log::info;
    use test_log::test;

    #[test]
    fn points() {
        let cases = vec![(DataPoints::X250Points, 250usize)];

        for (data_size_index, want) in cases {
            info!("Getting number of points for {data_size_index:?}, expecting {want}");
            let got = data_size_index.points();
            assert_eq!(want, got);
        }
    }

    #[test]
    fn default() {
        let want = DataPoints::X250Points;
        let got = DataPoints::default();
        assert_eq!(got, want, "it should default to {want:?}");
    }

    #[test]
    fn display() {
        let cases = vec![(DataPoints::X250Points, "250 points")];

        for (data_size_index, want) in cases {
            info!("Displaying {data_size_index:?}, expecting {want:?}");
            let got = format!("{data_size_index}");
            assert_eq!(want, got);
        }
    }

    #[test]
    fn ord() {
        let cases = vec![(DataPoints::X250Points, DataPoints::X250Points, Ordering::Equal)];

        for (data_size_index_1, data_size_index_2, want) in cases {
            info!("Ordering {data_size_index_1:?} against {data_size_index_2:?}, want {want:?}");
            let got = data_size_index_1.cmp(&data_size_index_2);
            assert_eq!(want, got);

            let got = data_size_index_1.partial_cmp(&data_size_index_2);
            assert!(got.is_some());
            assert_eq!(want, got.unwrap());
        }
    }
}
