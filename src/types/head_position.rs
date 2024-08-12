use crate::types::{
    util::primitive::{read_u8_bits, write_u8_bits},
    Direction,
};
use binrw::{BinRead, BinResult, BinWrite, Endian, Error};
use num_traits::ToPrimitive;
use std::cmp::Ordering;
use std::io::{Error as IOError, ErrorKind::InvalidData, Read, Seek, Write};

#[derive(Debug, Clone, derive_new::new)]
#[cfg_attr(
    target_family = "wasm",
    derive(tsify::Tsify, serde::Serialize, serde::Deserialize),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct HeadPosition {
    pub angle: f32,
    pub direction: Direction,
}

impl HeadPosition {
    pub const MIN: f32 = -180.0;
    pub const MAX: f32 = 180.0;

    const FLAG_DIRECTION: u8 = 0b0100_0000;
    const SHIFT_DIRECTION: usize = 6;

    #[inline]
    pub fn valid(&self) -> bool {
        Self::MIN <= self.angle && self.angle <= Self::MAX
    }
}

impl Default for HeadPosition {
    fn default() -> Self {
        Self { angle: 0.0, direction: Direction::default() }
    }
}

impl PartialEq for HeadPosition {
    fn eq(&self, other: &Self) -> bool {
        self.angle.eq(&other.angle)
    }
}

impl PartialOrd for HeadPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.angle.partial_cmp(&other.angle)
    }
}

impl BinRead for HeadPosition {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<Self> {
        let raw = u16::read_options(reader, endian, ())?;
        let pos = reader.stream_position()?;

        // extract the high and low parts
        let high = (raw >> 8) as u8;
        let low = (raw & 0xFF) as u8;

        // extract the direction enum
        let direction =
            read_u8_bits::<Direction>(low, Self::FLAG_DIRECTION, Self::SHIFT_DIRECTION, pos)?;

        // assemble the value
        let high_part = (low & 0b0011_1110) >> 1;
        let low_part = (high & 0b0111_1111) | (low & 0b1) << 7;
        let value = ((high_part as u16) << 8) | (low_part as u16);

        // convert the value to an angle
        let angle = 0.3 * (value as f32 - 600.0);

        Ok(Self { angle, direction })
    }
}

impl BinWrite for HeadPosition {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        let value = ((self.angle / 0.3).round() as u16).saturating_add(600);

        // extract the high and low parts
        let high = ((value >> 8) & 0xFF) as u8;
        let low = (value & 0xFF) as u8;

        // prepare the high byte
        let pos = writer.stream_position()?;
        let mut high = write_u8_bits(high, 0b0011_1110, 1, pos)?;
        high |= (low >> 7) & 0b1;

        let raw_direction = self.direction.to_u8().ok_or_else(|| Error::Custom {
            pos,
            err: Box::new(IOError::new(InvalidData, "Invalid direction value")),
        })?;

        high |= write_u8_bits(raw_direction, Self::FLAG_DIRECTION, Self::SHIFT_DIRECTION, pos)?;

        let low = low & 0b0111_1111;

        // combine the high and low parts into a single u16
        let raw = ((low as u16) << 8) | high as u16;

        raw.write_options(writer, endian, args)?;
        Ok(())
    }
}
