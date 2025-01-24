use crate::types::primitive::i14f2;
use binrw::{BinRead, BinResult, BinWrite, Endian};
use std::cmp::Ordering;
use std::io::{Read, Seek, Write};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, derive_new::new)]
#[cfg_attr(
    target_family = "wasm",
    derive(tsify::Tsify, serde::Serialize, serde::Deserialize),
    tsify(into_wasm_abi, from_wasm_abi)
)]
#[cfg_attr(
    all(feature = "serde", not(target_family = "wasm")),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(feature = "pyo3", pyclass(eq))]
pub struct Angle {
    pub angle: f32,
    new_data: bool,
    error_alarm: bool,
}

impl Angle {
    const SCALE: f32 = 0.025;
    const MAX: f32 = i14f2::MAX as f32 * Self::SCALE;
    const MIN: f32 = i14f2::MIN as f32 * Self::SCALE;

    pub fn valid_angle(angle: f32) -> bool {
        Self::MIN <= angle && angle <= Self::MAX
    }

    pub fn valid(&self) -> bool {
        Self::valid_angle(self.angle)
    }
}

impl PartialOrd<Self> for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.angle.partial_cmp(&other.angle)
    }
}

impl From<f32> for Angle {
    fn from(angle: f32) -> Self {
        Self { angle, new_data: false, error_alarm: false }
    }
}

impl From<f64> for Angle {
    fn from(angle: f64) -> Self {
        Self { angle: angle as f32, new_data: false, error_alarm: false }
    }
}

impl BinRead for Angle {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let (raw, new_data, error_alarm) = i14f2::parse(reader, endian, args)?;
        let angle = raw as f32 * Self::SCALE;
        Ok(Self::new(angle, new_data, error_alarm))
    }
}

impl BinWrite for Angle {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        let raw = (self.angle / Self::SCALE) as i16;
        let values = (raw, self.new_data, self.error_alarm);
        i14f2::write(&values, writer, endian, args)
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl Angle {
    #[new]
    pub fn py_new(angle: f32) -> Self {
        Angle { angle, new_data: false, error_alarm: false }
    }

    #[pyo3(name = "is_valid")]
    pub fn py_is_valid(&self) -> bool {
        Self::valid_angle(self.angle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    use log::info;
    use test_log::test;

    use binrw::Endian;

    const BINARY_ENDIAN: Endian = Endian::Little;
    const BINARY_CASES: [(Angle, [u8; 2]); 2] = [
        (
            Angle { angle: Angle::MIN, new_data: true, error_alarm: true },
            [0b1110_0000, 0b0000_0000],
        ),
        (
            Angle { angle: Angle::MAX, new_data: false, error_alarm: false },
            [0b0001_1111, 0b1111_1111],
        ),
    ];

    #[test]
    fn test_valid_angle() {
        let cases = vec![
            (Angle::MAX, true),
            (Angle::MIN, true),
            (Angle::MAX + 1.0, false),
            (Angle::MIN - 1.0, false),
        ];

        for (angle, want) in cases {
            info!("Testing validity of {angle:?}, expecting {want:?}");
            let got = Angle::valid_angle(angle);
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_valid() {
        let cases = vec![
            (Angle::new(Angle::MAX, true, false), true),
            (Angle::new(Angle::MIN, false, true), true),
            (Angle::new(Angle::MAX + 1.0, true, false), false),
            (Angle::new(Angle::MIN - 1.0, false, false), false),
        ];

        for (angle, want) in cases {
            info!("Testing validity of {angle:?}, expecting {want:?}");
            let got = angle.valid();
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_parse() {
        for &(want, ref bytes) in BINARY_CASES.iter() {
            info!("Parsing {bytes:?}, expecting {want:?}");
            let mut cursor = Cursor::new(bytes);
            let got = Angle::read_options(&mut cursor, BINARY_ENDIAN, ())
                .expect("Should not return an error");
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_write() {
        for &(angle, ref want) in BINARY_CASES.iter() {
            info!("Writing {angle:?}, expecting {want:?}");
            let mut cursor = Cursor::new(Vec::new());
            angle
                .write_options(&mut cursor, BINARY_ENDIAN, ())
                .expect("Should not return an error");
            let got_inner = cursor.into_inner();
            let got = got_inner.as_slice();
            assert_eq!(want, got);
        }
    }
}
