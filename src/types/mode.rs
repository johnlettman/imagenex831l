use binrw::{BinRead, BinWrite};
use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Copy, Clone, BinRead, BinWrite, FromPrimitive, ToPrimitive)]
#[repr(u8)]
#[brw(repr = u8)]
#[cfg_attr(
    target_family = "wasm",
    derive(tsify::Tsify, serde::Serialize, serde::Deserialize),
    tsify(into_wasm_abi, from_wasm_abi),
    serde(rename_all = "UPPERCASE")
)]
pub enum Mode {
    Sector = 0,
    Polar = 1,
    Sidescan = 2,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Sector => "sector",
                Self::Polar => "polar",
                Self::Sidescan => "sidescan",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let cases =
            vec![(Mode::Sector, "sector"), (Mode::Polar, "polar"), (Mode::Sidescan, "sidescan")];

        for (mode, want) in cases {
            let got = format!("{mode}");
            assert_eq!(want, got);
        }
    }
}
