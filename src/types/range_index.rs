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
    #[cfg_attr(target_family = "wasm", serde(rename = "12.5cm"))]
    X12_5cm = 2,

    #[cfg_attr(target_family = "wasm", serde(rename = "25.0cm"))]
    X25cm = 4,

    #[cfg_attr(target_family = "wasm", serde(rename = "50.0cm"))]
    X50cm = 6,

    #[cfg_attr(target_family = "wasm", serde(rename = "75.0cm"))]
    X75cm = 8,

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
            Self::X12_5cm => 0.125,
            Self::X25cm => 0.250,
            Self::X50cm => 0.500,
            Self::X75cm => 0.750,
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
            Self::X12_5cm | Self::X25cm | Self::X50cm | Self::X75cm => 0.016,
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
            (RangeIndex::X12_5cm, 0.125),
            (RangeIndex::X25cm, 0.250),
            (RangeIndex::X50cm, 0.500),
            (RangeIndex::X75cm, 0.750),
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
    fn test_display() {
        let cases = vec![
            (RangeIndex::X12_5cm, "0.125 meters"),
            (RangeIndex::X25cm, "0.250 meters"),
            (RangeIndex::X50cm, "0.500 meters"),
            (RangeIndex::X75cm, "0.750 meters"),
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
