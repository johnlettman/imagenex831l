use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Copy, Clone, FromPrimitive, ToPrimitive)]
#[repr(u8)]
#[cfg_attr(
    target_family = "wasm",
    derive(tsify::Tsify, serde::Serialize, serde::Deserialize),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub enum DataBits {
    #[cfg_attr(target_family = "wasm", serde(rename = "4_BITS"))]
    X4Bits = 0,

    #[cfg_attr(target_family = "wasm", serde(rename = "8_BITS"))]
    X8Bits = 1,

    #[cfg_attr(target_family = "wasm", serde(rename = "14_BITS"))]
    X14Bits = 2,
}

impl Display for DataBits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::X4Bits => "4 bits",
                Self::X8Bits => "8 bits",
                Self::X14Bits => "14 bits",
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
    fn test_display() {
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
}
