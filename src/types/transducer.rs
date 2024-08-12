use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Copy, Clone, FromPrimitive, ToPrimitive)]
#[repr(u8)]
#[cfg_attr(
    target_family = "wasm",
    derive(tsify::Tsify, serde::Serialize, serde::Deserialize),
    tsify(into_wasm_abi, from_wasm_abi),
    serde(rename_all = "UPPERCASE")
)]
pub enum Transducer {
    Down = 0,
    Up = 1,
}

impl Display for Transducer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Down => "down",
                Self::Up => "up",
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
        let cases = vec![(Transducer::Down, "down"), (Transducer::Up, "up")];

        for (transducer, want) in cases {
            info!("Displaying {transducer:?}, expecting {want:?}");
            let got = format!("{transducer}");
            assert_eq!(want, got);
        }
    }
}
