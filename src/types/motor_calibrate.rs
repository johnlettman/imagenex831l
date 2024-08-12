use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Copy, Clone, ToPrimitive, FromPrimitive)]
#[repr(u8)]
#[cfg_attr(
    target_family = "wasm",
    derive(tsify::Tsify, serde::Serialize, serde::Deserialize),
    tsify(into_wasm_abi, from_wasm_abi),
    serde(rename_all = "UPPERCASE")
)]
pub enum MotorCalibrate {
    Normal = 0,
    Calibrate = 1,
}

impl Default for MotorCalibrate {
    fn default() -> Self {
        Self::Normal
    }
}

impl Display for MotorCalibrate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Normal => "normal operation",
                Self::Calibrate => "calibrate sonar head transducer",
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
        let got = MotorCalibrate::default();
        assert_eq!(MotorCalibrate::Normal, got);
    }

    #[test]
    fn test_display() {
        let cases = vec![
            (MotorCalibrate::Normal, "normal operation"),
            (MotorCalibrate::Calibrate, "calibrate sonar head transducer"),
        ];

        for (motor_calibrate, want) in cases {
            info!("Displaying {motor_calibrate:?}, expecting {want:?}");
            let got = format!("{motor_calibrate}");
            assert_eq!(want, got);
        }
    }
}
