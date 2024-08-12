use binrw::{BinRead, BinWrite};
use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Copy, Clone, BinRead, BinWrite, ToPrimitive, FromPrimitive)]
#[repr(u8)]
#[brw(repr = u8)]
#[cfg_attr(
    target_family = "wasm",
    derive(tsify::Tsify, serde::Serialize, serde::Deserialize),
    tsify(into_wasm_abi, from_wasm_abi),
    serde(rename_all = "UPPERCASE")
)]
pub enum Reverse {
    Normal = 0b0000_0000,
    Reverse = 0b0100_0000,
}

impl Default for Reverse {
    fn default() -> Self {
        Self::Normal
    }
}

impl Display for Reverse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Normal => "normal",
                Self::Reverse => "reverse step direction",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use log::info;
    use test_log::test;

    #[test]
    fn test_default() {
        let got = Reverse::default();
        assert_eq!(Reverse::Normal, got);
    }

    #[test]
    fn test_display() {
        let cases = vec![(Reverse::Normal, "normal"), (Reverse::Reverse, "reverse step direction")];

        for (reverse, want) in cases {
            info!("Displaying {reverse:?}, expecting {want:?}");
            let got = format!("{reverse}");
            assert_eq!(want, got);
        }
    }
}
