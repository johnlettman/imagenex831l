use crate::types::primitive::i14f2;
use binrw::{BinRead, BinResult, BinWrite, Endian};
use std::cmp::Ordering;
use std::io::{Read, Seek, Write};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[derive(Debug, Clone, derive_new::new)]
#[cfg_attr(
    target_family = "wasm",
    derive(tsify::Tsify, serde::Serialize, serde::Deserialize),
    tsify(into_wasm_abi, from_wasm_abi)
)]
#[cfg_attr(
    all(feature = "serde", not(target_family = "wasm")),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(feature = "pyo3", pyclass(eq, ord))]
pub struct Acceleration {
    pub acceleration: f32,
    new_data: bool,
    error_alarm: bool,
}

impl Acceleration {
    const SCALE: f32 = 0.24414;
}

impl PartialEq for Acceleration {
    fn eq(&self, other: &Self) -> bool {
        self.acceleration.eq(&other.acceleration)
    }
}

impl PartialOrd for Acceleration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.acceleration.partial_cmp(&other.acceleration)
    }
}

impl From<f32> for Acceleration {
    fn from(acceleration: f32) -> Self {
        Self { acceleration, new_data: false, error_alarm: false }
    }
}

impl From<f64> for Acceleration {
    fn from(acceleration: f64) -> Self {
        Self { acceleration: acceleration as f32, new_data: false, error_alarm: false }
    }
}

impl BinRead for Acceleration {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let (raw, new_data, error_alarm) = i14f2::parse(reader, endian, args)?;
        let acceleration = raw as f32 * Acceleration::SCALE;
        Ok(Self::new(acceleration, new_data, error_alarm))
    }
}

impl BinWrite for Acceleration {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        let raw = (self.acceleration / Acceleration::SCALE) as i16;
        let values = (raw, self.new_data, self.error_alarm);
        i14f2::write(&values, writer, endian, args)
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl Acceleration {
    #[new]
    pub fn py_new(acceleration: f32) -> PyResult<Self> {
        Ok(Self { acceleration, new_data: false, error_alarm: false })
    }

    pub fn __repr__(&self) -> String {
        format!(
            "Acceleration({}, new_data = {}, error_alarm = {})",
            self.acceleration, self.new_data, self.error_alarm
        )
    }

    pub fn __str__(&self) -> String {
        self.acceleration.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use log::info;
    use test_log::test;

    #[test]
    fn eq() {
        let cases = vec![
            (Acceleration::new(6.0, true, false), Acceleration::new(6.0, true, false), true),
            (Acceleration::new(12.0, true, false), Acceleration::new(12.0, true, false), true),
            (Acceleration::new(32.0, true, false), Acceleration::new(60.0, true, false), false),
            (Acceleration::new(86.0, true, false), Acceleration::new(62.0, true, false), false),
        ];

        for (a, b, want) in cases {
            info!("Testing equality between {a:?} and {b:?}, expecting {want:?}");
            assert_eq!(a.eq(&b), want);
        }
    }

    #[test]
    fn ord() {
        let cases = vec![
            (
                Acceleration::new(6.0, true, false),
                Acceleration::new(6.0, true, false),
                Some(Ordering::Equal),
            ),
            (
                Acceleration::new(12.0, true, false),
                Acceleration::new(12.0, true, false),
                Some(Ordering::Equal),
            ),
            (
                Acceleration::new(32.0, true, false),
                Acceleration::new(60.0, true, false),
                Some(Ordering::Less),
            ),
            (
                Acceleration::new(86.0, true, false),
                Acceleration::new(62.0, true, false),
                Some(Ordering::Greater),
            ),
        ];

        for (a, b, want) in cases {
            info!("Comparing {a:?} and {b:?}, expecting {want:?}");
            assert_eq!(a.partial_cmp(&b), want);
        }
    }

    #[test]
    fn from_f32() {
        assert_eq!(Acceleration::from(1.0), Acceleration::new(1.0, false, false));
    }

    #[test]
    fn from_f64() {
        assert_eq!(Acceleration::from(1.0), Acceleration::new(1.0, false, false));
    }
}
