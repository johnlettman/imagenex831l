use binrw::{BinRead, BinWrite};
use std::fmt::{Display, Formatter};

#[derive(Debug, BinRead, BinWrite, Eq, PartialEq, Copy, Clone)]
#[brw(repr = u8)]
#[cfg_attr(
    target_family = "wasm",
    derive(tsify::Tsify, serde::Serialize, serde::Deserialize),
    tsify(into_wasm_abi, from_wasm_abi),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum SensorAvailable {
    NotPresent = 0,
    Present = 1,
}

impl Default for SensorAvailable {
    fn default() -> Self {
        Self::NotPresent
    }
}

impl Display for SensorAvailable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                SensorAvailable::NotPresent => "no external pitch / roll / distance sensor present",
                SensorAvailable::Present => "external pitch / roll / distance sensor present",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let got = SensorAvailable::default();
        assert_eq!(got, SensorAvailable::NotPresent);
    }

    #[test]
    fn test_display() {
        let cases = vec![
            (SensorAvailable::NotPresent, "no external pitch / roll / distance sensor present"),
            (SensorAvailable::Present, "external pitch / roll / distance sensor present"),
        ];

        for (sensor_available, want) in cases {
            let got = format!("{sensor_available}");
            assert_eq!(got, want);
        }
    }
}
