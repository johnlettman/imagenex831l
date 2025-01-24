use binrw::meta::{EndianKind, ReadEndian, WriteEndian};
use binrw::{BinRead, BinResult, BinWrite, Endian};
use std::fmt::Display;
use std::io::{Read, Seek, Write};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[derive(Debug, Default, Eq, PartialEq, Clone, derive_new::new)]
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
pub struct SonarReturnStatus {
    #[cfg(not(feature = "pyo3"))]
    pub range_error: bool,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    pub range_error: bool,

    #[cfg(not(feature = "pyo3"))]
    pub frequency_error: bool,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    pub frequency_error: bool,

    #[cfg(not(feature = "pyo3"))]
    pub internal_sensor_error: bool,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    pub internal_sensor_error: bool,

    #[cfg(not(feature = "pyo3"))]
    pub calibration_error: bool,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    pub calibration_error: bool,

    #[cfg(not(feature = "pyo3"))]
    pub switches_accepted: bool,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    pub switches_accepted: bool,
}

impl SonarReturnStatus {
    const FLAG_RANGE_ERROR: u8 = 0b1000_0000;
    const FLAG_FREQUENCY_ERROR: u8 = 0b0100_0000;
    const FLAG_INTERNAL_SENSOR_ERROR: u8 = 0b0010_0000;
    const FLAG_CALIBRATION_ERROR: u8 = 0b0001_0000;
    const FLAG_SWITCHES_ACCEPTED: u8 = 0b0000_0001;

    pub fn has_error(&self) -> bool {
        self.range_error
            || self.frequency_error
            || self.internal_sensor_error
            || self.calibration_error
    }
}

impl Display for SonarReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut messages = Vec::new();

        if self.has_error() {
            if self.range_error {
                messages.push("range error");
            }

            if self.frequency_error {
                messages.push("frequency error");
            }

            if self.internal_sensor_error {
                messages.push("internal sensor error");
            }

            if self.calibration_error {
                messages.push("calibration error");
            }
        } else {
            messages.push("no errors");
        }

        if self.switches_accepted {
            messages.push("switches accepted");
        }

        let message = messages.join(", ");
        write!(f, "{}", message)
    }
}

impl ReadEndian for SonarReturnStatus {
    const ENDIAN: EndianKind = EndianKind::None;
}

impl WriteEndian for SonarReturnStatus {
    const ENDIAN: EndianKind = EndianKind::None;
}

impl BinRead for SonarReturnStatus {
    type Args<'a> = ();

    fn read<R: Read + Seek>(reader: &mut R) -> BinResult<Self>
    where
        Self: ReadEndian,
    {
        let raw = u8::read(reader)?;

        let range_error = raw & Self::FLAG_RANGE_ERROR != 0;
        let frequency_error = raw & Self::FLAG_FREQUENCY_ERROR != 0;
        let internal_sensor_error = raw & Self::FLAG_INTERNAL_SENSOR_ERROR != 0;
        let calibration_error = raw & Self::FLAG_CALIBRATION_ERROR != 0;
        let switches_accepted = raw & Self::FLAG_SWITCHES_ACCEPTED != 0;

        Ok(Self {
            range_error,
            frequency_error,
            internal_sensor_error,
            calibration_error,
            switches_accepted,
        })
    }

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<Self> {
        Self::read(reader)
    }
}

impl BinWrite for SonarReturnStatus {
    type Args<'a> = ();

    fn write<W: Write + Seek>(&self, writer: &mut W) -> BinResult<()>
    where
        Self: WriteEndian,
    {
        let mut raw: u8 = 0;

        if self.range_error {
            raw |= Self::FLAG_RANGE_ERROR;
        }

        if self.frequency_error {
            raw |= Self::FLAG_FREQUENCY_ERROR;
        }

        if self.internal_sensor_error {
            raw |= Self::FLAG_INTERNAL_SENSOR_ERROR;
        }

        if self.calibration_error {
            raw |= Self::FLAG_CALIBRATION_ERROR;
        }

        if self.switches_accepted {
            raw |= Self::FLAG_SWITCHES_ACCEPTED;
        }

        raw.write(writer)
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
impl SonarReturnStatus {
    #[new]
    pub(crate) fn py_new(
        range_error: bool,
        frequency_error: bool,
        internal_sensor_error: bool,
        calibration_error: bool,
        switches_accepted: bool,
    ) -> Self {
        Self::new(
            range_error,
            frequency_error,
            internal_sensor_error,
            calibration_error,
            switches_accepted,
        )
    }

    pub(crate) fn __str__(&self) -> String {
        self.to_string()
    }

    pub(crate) fn __repr__(&self) -> String {
        format!(
            "SonarReturnStatus(\
        range_error={:?}, \
        frequency_error={:?}, \
        internal_sensor_error={:?}, \
        calibration_error={:?}, \
        switches_accepted={:?})",
            self.range_error,
            self.frequency_error,
            self.internal_sensor_error,
            self.calibration_error,
            self.switches_accepted
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::{io::Cursor, BinRead, BinWrite};

    use log::info;
    use test_log::test;

    #[test]
    fn test_default() {
        let want = SonarReturnStatus {
            range_error: false,
            frequency_error: false,
            internal_sensor_error: false,
            calibration_error: false,
            switches_accepted: false,
        };

        let got = SonarReturnStatus::default();
        assert_eq!(want, got);
    }

    #[test]
    fn test_has_error() {
        let mut status = SonarReturnStatus::default();
        assert!(!status.has_error());

        status.range_error = true;
        assert!(status.has_error());

        status.range_error = false;
        status.frequency_error = true;
        assert!(status.has_error());

        status.internal_sensor_error = true;
        assert!(status.has_error());

        status.calibration_error = true;
        assert!(status.has_error());
    }

    const BINARY_ENDIAN: Endian = Endian::Big;
    const BINARY_CASES: [(SonarReturnStatus, u8); 5] = [
        (
            SonarReturnStatus {
                range_error: false,
                frequency_error: false,
                internal_sensor_error: false,
                calibration_error: false,
                switches_accepted: false,
            },
            0b0000_0000,
        ),
        (
            SonarReturnStatus {
                range_error: true,
                frequency_error: false,
                internal_sensor_error: false,
                calibration_error: false,
                switches_accepted: true,
            },
            0b1000_0001,
        ),
        (
            SonarReturnStatus {
                range_error: true,
                frequency_error: true,
                internal_sensor_error: true,
                calibration_error: true,
                switches_accepted: true,
            },
            0b1111_0001,
        ),
        (
            SonarReturnStatus {
                range_error: false,
                frequency_error: true,
                internal_sensor_error: false,
                calibration_error: true,
                switches_accepted: false,
            },
            0b0101_0000,
        ),
        (
            SonarReturnStatus {
                range_error: true,
                frequency_error: true,
                internal_sensor_error: false,
                calibration_error: false,
                switches_accepted: false,
            },
            0b1100_0000,
        ),
    ];

    #[test]
    fn test_parse() {
        for &(ref want, raw) in BINARY_CASES.iter() {
            info!("Parsing {raw:?}, expecting {want:?}");
            let mut cursor = Cursor::new(vec![raw]);
            let got = SonarReturnStatus::read_options(&mut cursor, BINARY_ENDIAN, ())
                .expect("It should not return an error");
            assert_eq!(want, &got);
        }
    }

    #[test]
    fn test_write() {
        for &(ref status, raw) in BINARY_CASES.iter() {
            info!("Writing {status:?}, expecting {raw:?}");
            let mut cursor = Cursor::new(Vec::new());
            status
                .write_options(&mut cursor, BINARY_ENDIAN, ())
                .expect("It should not return an error");
            let written_data = cursor.into_inner();
            assert_eq!(written_data, vec![raw]);
        }
    }
}
