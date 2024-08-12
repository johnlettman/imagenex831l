use crate::types::primitive::i14f2;
use binrw::{BinRead, BinResult, BinWrite, Endian};
use std::io::{Read, Seek, Write};

#[derive(Debug, Clone, derive_new::new)]
pub struct Angle {
    angle: f32,
    new_data: bool,
    error_alarm: bool,
}

impl Angle {
    const SCALE: f32 = 0.025;
}

impl BinRead for Angle {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let (raw, new_data, error_alarm) = i14f2::parse(reader, endian, args)?;
        let angle = raw as f32 * Angle::SCALE;
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
        let raw = (self.angle / Angle::SCALE) as i16;
        let values = (raw, self.new_data, self.error_alarm);
        i14f2::write(&values, writer, endian, args)
    }
}
