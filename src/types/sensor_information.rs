use binrw::meta::{EndianKind, ReadEndian, WriteEndian};
use binrw::{BinRead, BinResult, BinWrite, Endian};
use std::fmt::{Display, Formatter};
use std::io::{Read, Seek, Write};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

const FLAG_PITCH_VALID: u8 = 0b0000_0001;
const FLAG_ROLL_VALID: u8 = 0b0000_0010;
const FLAG_DISTANCE_VALID: u8 = 0b0000_0100;

#[derive(Debug, Eq, PartialEq, Copy, Clone, derive_new::new)]
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
pub struct SensorInformation {
    #[cfg(not(feature = "pyo3"))]
    pub pitch_valid: bool,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    pub pitch_valid: bool,

    #[cfg(not(feature = "pyo3"))]
    pub roll_valid: bool,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    pub roll_valid: bool,

    #[cfg(not(feature = "pyo3"))]
    pub distance_valid: bool,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    pub distance_valid: bool,
}

impl SensorInformation {
    #[inline]
    const fn fmt_valid(valid: bool) -> &'static str {
        if valid {
            "valid"
        } else {
            "invalid"
        }
    }
}

impl Default for SensorInformation {
    fn default() -> Self {
        Self { pitch_valid: true, roll_valid: true, distance_valid: true }
    }
}

impl Display for SensorInformation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(pitch: {}, roll: {}, distance: {})",
            Self::fmt_valid(self.pitch_valid),
            Self::fmt_valid(self.roll_valid),
            Self::fmt_valid(self.distance_valid)
        )
    }
}

impl ReadEndian for SensorInformation {
    const ENDIAN: EndianKind = EndianKind::None;
}

impl WriteEndian for SensorInformation {
    const ENDIAN: EndianKind = EndianKind::None;
}

impl BinRead for SensorInformation {
    type Args<'a> = ();

    fn read<R: Read + Seek>(reader: &mut R) -> BinResult<Self>
    where
        Self: ReadEndian,
    {
        let raw = u8::read(reader)?;
        Ok(Self {
            pitch_valid: (raw & FLAG_PITCH_VALID) != 0,
            roll_valid: (raw & FLAG_ROLL_VALID) != 0,
            distance_valid: (raw & FLAG_DISTANCE_VALID) != 0,
        })
    }

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<Self> {
        SensorInformation::read(reader)
    }
}

impl BinWrite for SensorInformation {
    type Args<'a> = ();

    fn write<W: Write + Seek>(&self, writer: &mut W) -> BinResult<()>
    where
        Self: WriteEndian,
    {
        let mut raw: u8 = 0;

        if self.pitch_valid {
            raw |= FLAG_PITCH_VALID;
        }

        if self.roll_valid {
            raw |= FLAG_ROLL_VALID;
        }

        if self.distance_valid {
            raw |= FLAG_DISTANCE_VALID;
        }

        raw.write(writer)?;
        Ok(())
    }

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        _: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<()> {
        self.write(writer)
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl SensorInformation {
    #[new]
    pub(crate) fn py_new(pitch_valid: bool, roll_valid: bool, distance_valid: bool) -> Self {
        Self::new(pitch_valid, roll_valid, distance_valid)
    }

    pub(crate) fn __str__(&self) -> String {
        self.to_string()
    }

    pub(crate) fn __repr__(&self) -> String {
        format!(
            "SensorInformation(pitch_valid={:?}, roll_valid={:?}, distance_valid={:?})",
            self.pitch_valid, self.roll_valid, self.distance_valid
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    use log::info;
    use test_log::test;

    #[test]
    fn fmt_valid() {
        let cases = vec![(true, "valid"), (false, "invalid")];

        for (validity, want) in cases {
            info!("Formatting {validity:?}, want {want:?}");
            let got = SensorInformation::fmt_valid(validity);
            assert_eq!(want, got);
        }
    }

    #[test]
    fn display() {
        let cases = vec![
            (
                SensorInformation { pitch_valid: true, roll_valid: true, distance_valid: true },
                "(pitch: valid, roll: valid, distance: valid)",
            ),
            (
                SensorInformation { pitch_valid: true, roll_valid: false, distance_valid: true },
                "(pitch: valid, roll: invalid, distance: valid)",
            ),
            (
                SensorInformation { pitch_valid: true, roll_valid: true, distance_valid: false },
                "(pitch: valid, roll: valid, distance: invalid)",
            ),
            (
                SensorInformation { pitch_valid: false, roll_valid: true, distance_valid: true },
                "(pitch: invalid, roll: valid, distance: valid)",
            ),
        ];

        for (sensor_information, want) in cases {
            info!("Displaying {sensor_information:?}, want {want:?}");
            let got = format!("{sensor_information}");
            assert_eq!(want, got);
        }
    }

    const BINARY_ENDIAN: Endian = Endian::NATIVE;
    const BINARY_CASES: [(SensorInformation, [u8; 1]); 5] = [
        (
            SensorInformation { pitch_valid: true, roll_valid: true, distance_valid: true },
            [0b0000_0111],
        ),
        (
            SensorInformation { pitch_valid: true, roll_valid: true, distance_valid: false },
            [0b0000_0011],
        ),
        (
            SensorInformation { pitch_valid: true, roll_valid: false, distance_valid: true },
            [0b0000_0101],
        ),
        (
            SensorInformation { pitch_valid: false, roll_valid: true, distance_valid: true },
            [0b0000_0110],
        ),
        (
            SensorInformation { pitch_valid: false, roll_valid: false, distance_valid: false },
            [0b0000_0000],
        ),
    ];

    #[test]
    fn parse() {
        for (want, bytes) in BINARY_CASES {
            info!("Parsing {bytes:?}, want {want:?}");
            let mut cursor = Cursor::new(bytes);
            let got = SensorInformation::read(&mut cursor).expect("It should not return an error");
            assert_eq!(want, got);
        }
    }

    #[test]
    fn parse_options() {
        for (want, bytes) in BINARY_CASES {
            info!("Parsing {bytes:?}, want {want:?}");
            let mut cursor = Cursor::new(bytes);
            let got = SensorInformation::read_options(&mut cursor, BINARY_ENDIAN, ())
                .expect("It should not return an error");
            assert_eq!(want, got);
        }
    }

    #[test]
    fn write() {
        for (sensor_information, want) in BINARY_CASES {
            info!("Writing {sensor_information:?}, want {want:?}");
            let mut cursor = Cursor::new(Vec::new());
            sensor_information.write(&mut cursor).expect("It should not return an error");
            let inner = cursor.into_inner();
            let got = inner.as_slice();
            assert_eq!(want, got);
        }
    }

    #[test]
    fn write_options() {
        for (sensor_information, want) in BINARY_CASES {
            info!("Writing {sensor_information:?}, want {want:?}");
            let mut cursor = Cursor::new(Vec::new());
            sensor_information
                .write_options(&mut cursor, BINARY_ENDIAN, ())
                .expect("It should not return an error");
            let inner = cursor.into_inner();
            let got = inner.as_slice();
            assert_eq!(want, got);
        }
    }
}
