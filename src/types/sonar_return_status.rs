use binrw::meta::{EndianKind, ReadEndian, WriteEndian};
use binrw::{BinRead, BinResult, BinWrite, Endian};
use std::io::{Read, Seek, Write};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SonarReturnStatus {
    pub range_error: bool,
    pub frequency_error: bool,
    pub internal_sensor_error: bool,
    pub calibration_error: bool,
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

impl Default for SonarReturnStatus {
    fn default() -> Self {
        Self {
            range_error: false,
            frequency_error: false,
            internal_sensor_error: false,
            calibration_error: false,
            switches_accepted: false,
        }
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
