use binrw::{BinRead, BinWrite};
use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt::{Display, Formatter};

#[derive(
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Copy,
    Clone,
    ToPrimitive,
    FromPrimitive,
    BinRead,
    BinWrite,
)]
#[repr(u8)]
#[brw(repr = u8)]
#[cfg_attr(
    target_family = "wasm",
    derive(tsify::Tsify, serde::Serialize, serde::Deserialize),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub enum RangeIndex {
    #[cfg_attr(target_family = "wasm", serde(rename = "0.125m"))]
    X0_125m = 2,

    #[cfg_attr(target_family = "wasm", serde(rename = "0.25m"))]
    X0_25m = 4,

    #[cfg_attr(target_family = "wasm", serde(rename = "0.50m"))]
    X0_50m = 6,

    #[cfg_attr(target_family = "wasm", serde(rename = "0.75m"))]
    X0_75m = 8,

    #[cfg_attr(target_family = "wasm", serde(rename = "1.0m"))]
    X1m = 10,

    #[cfg_attr(target_family = "wasm", serde(rename = "2.0m"))]
    X2m = 20,

    #[cfg_attr(target_family = "wasm", serde(rename = "3.0m"))]
    X3m = 30,

    #[cfg_attr(target_family = "wasm", serde(rename = "4.0m"))]
    X4m = 40,

    #[cfg_attr(target_family = "wasm", serde(rename = "5.0m"))]
    X5m = 50,

    #[cfg_attr(target_family = "wasm", serde(rename = "6.0m"))]
    X6m = 60,
}

impl RangeIndex {
    pub fn range(&self) -> f32 {
        match *self {
            Self::X0_125m => 0.125,
            Self::X0_25m => 0.250,
            Self::X0_50m => 0.500,
            Self::X0_75m => 0.750,
            Self::X1m => 1.0,
            Self::X2m => 2.0,
            Self::X3m => 3.0,
            Self::X4m => 4.0,
            Self::X5m => 5.0,
            Self::X6m => 6.0,
        }
    }

    pub fn filter_delay(&self) -> f32 {
        match *self {
            Self::X0_125m | Self::X0_25m | Self::X0_50m | Self::X0_75m => 0.016,
            Self::X1m => 0.020,
            Self::X2m => 0.024,
            Self::X3m => 0.030,
            Self::X4m | Self::X5m | Self::X6m => 0.030,
        }
    }
}

impl Display for RangeIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let range = self.range();
        write!(f, "{range:.3} meters")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use log::info;
    use test_log::test;

    #[test]
    fn test_range() {
        let cases = vec![
            (RangeIndex::X0_125m, 0.125),
            (RangeIndex::X0_25m, 0.250),
            (RangeIndex::X0_50m, 0.500),
            (RangeIndex::X0_75m, 0.750),
            (RangeIndex::X1m, 1.0),
            (RangeIndex::X2m, 2.0),
            (RangeIndex::X3m, 3.0),
            (RangeIndex::X4m, 4.0),
            (RangeIndex::X5m, 5.0),
            (RangeIndex::X6m, 6.0),
        ];

        for (range, want) in cases {
            info!("Getting range for {range:?}, expecting {want}");
            let got = range.range();
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_filter_delay() {
        let cases = vec![
            (RangeIndex::X0_125m, 0.016),
            (RangeIndex::X0_25m, 0.016),
            (RangeIndex::X0_50m, 0.016),
            (RangeIndex::X0_75m, 0.016),
            (RangeIndex::X1m, 0.020),
            (RangeIndex::X2m, 0.024),
            (RangeIndex::X3m, 0.030),
            (RangeIndex::X4m, 0.030),
            (RangeIndex::X5m, 0.030),
            (RangeIndex::X6m, 0.030),
        ];

        for (range, want) in cases {
            info!("Getting filter delay for {range:?}, want {want:?}");
            let got = range.filter_delay();
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_display() {
        let cases = vec![
            (RangeIndex::X0_125m, "0.125 meters"),
            (RangeIndex::X0_25m, "0.250 meters"),
            (RangeIndex::X0_50m, "0.500 meters"),
            (RangeIndex::X0_75m, "0.750 meters"),
            (RangeIndex::X1m, "1.000 meters"),
            (RangeIndex::X2m, "2.000 meters"),
            (RangeIndex::X3m, "3.000 meters"),
            (RangeIndex::X4m, "4.000 meters"),
            (RangeIndex::X5m, "5.000 meters"),
            (RangeIndex::X6m, "6.000 meters"),
        ];

        for (range, want) in cases {
            info!("Displaying {range:?}, expecting {want}");
            let got = format!("{range}");
            assert_eq!(want, got);
        }
    }
}
