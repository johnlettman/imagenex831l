use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Copy, Clone, FromPrimitive, ToPrimitive)]
#[cfg_attr(
    target_family = "wasm",
    derive(tsify::Tsify, serde::Serialize, serde::Deserialize),
    tsify(into_wasm_abi, from_wasm_abi),
    serde(rename_all = "UPPERCASE")
)]
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

#[cfg(test)]
mod tests {
    use super::*;

    use log::info;
    use test_log::test;

    #[test]
    fn test_default() {
        let want = ProfileGrid::Off;
        let got = ProfileGrid::default();
        assert_eq!(want, got);
    }

    #[test]
    fn test_display() {
        let cases = vec![(ProfileGrid::Off, "off"), (ProfileGrid::On, "on")];

        for (profile_grid, want) in cases {
            info!("Displaying {profile_grid:?}, want {want}");
            let got = format!("{profile_grid}");
            assert_eq!(want, got);
        }
    }
}
