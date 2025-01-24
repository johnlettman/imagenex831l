use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt::{Display, Formatter};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone, ToPrimitive, FromPrimitive)]
#[repr(u8)]
#[cfg_attr(
    target_family = "wasm",
    derive(tsify::Tsify, serde::Serialize, serde::Deserialize),
    tsify(into_wasm_abi, from_wasm_abi)
)]
#[cfg_attr(
    all(feature = "serde", not(target_family = "wasm")),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "pyo3", pyclass(eq))]
pub enum MotorCalibrate {
    NoCalibrate = 0,
    Calibrate = 1,
}

impl Default for MotorCalibrate {
    fn default() -> Self {
        Self::NoCalibrate
    }
}

impl Display for MotorCalibrate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::NoCalibrate => "normal operation",
                Self::Calibrate => "calibrate sonar head transducer",
            }
        )
    }
}

impl From<bool> for MotorCalibrate {
    fn from(value: bool) -> Self {
        match value {
            false => Self::NoCalibrate,
            true => Self::Calibrate,
        }
    }
}

impl Into<bool> for MotorCalibrate {
    fn into(self) -> bool {
        self == Self::Calibrate
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl MotorCalibrate {
    #[new]
    pub(crate) fn py_new(calibrate: bool) -> Self {
        calibrate.into()
    }

    pub(crate) fn __str__(&self) -> String {
        self.to_string()
    }

    pub(crate) fn __bool__(&self) -> bool {
        (*self).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::{FromPrimitive, ToPrimitive};

    use log::info;
    use test_log::test;

    const PRIMITIVE_CASES: [(MotorCalibrate, u8); 2] =
        [(MotorCalibrate::NoCalibrate, 0), (MotorCalibrate::Calibrate, 1)];

    #[test]
    fn test_from_primitive() {
        for (want, primitive) in PRIMITIVE_CASES {
            info!("From primitive {primitive:?}, want {want:?}");
            let got = MotorCalibrate::from_u8(primitive).expect("It should return a value");
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_to_primitive() {
        for (motor_calibrate, want) in PRIMITIVE_CASES {
            info!("{motor_calibrate:?} to primitive, want {want:?}");
            let got = motor_calibrate.to_u8().expect("It should return a value");
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_default() {
        let got = MotorCalibrate::default();
        assert_eq!(MotorCalibrate::NoCalibrate, got);
    }

    #[test]
    fn test_display() {
        let cases = vec![
            (MotorCalibrate::NoCalibrate, "normal operation"),
            (MotorCalibrate::Calibrate, "calibrate sonar head transducer"),
        ];

        for (motor_calibrate, want) in cases {
            info!("Displaying {motor_calibrate:?}, expecting {want:?}");
            let got = format!("{motor_calibrate}");
            assert_eq!(want, got);
        }
    }
}
