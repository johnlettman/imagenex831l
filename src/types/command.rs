use crate::types::util::primitive::read_u8_bits;
use crate::types::{ProfilePointDetection, StepDirection};
use binrw::meta::{EndianKind, ReadEndian, WriteEndian};
use binrw::{BinRead, BinResult, Endian};
use std::io::{Read, Seek};

const MASK_STEP_DIRECTION: u8 = 0b0100_0000;
const SHIFT_STEP_DIRECTION: usize = 6;

const MASK_PROFILE_POINT_DETECTION: u8 = 0b0000_0001;

#[derive(Debug, Clone, Eq, PartialEq, derive_new::new)]
pub struct Command {
    pub profile_point_detection: ProfilePointDetection,
    pub step_direction: StepDirection,
}

impl ReadEndian for Command {
    const ENDIAN: EndianKind = EndianKind::None;
}

impl WriteEndian for Command {
    const ENDIAN: EndianKind = EndianKind::None;
}

impl BinRead for Command {
    type Args<'a> = ();

    fn read<R: Read + Seek>(reader: &mut R) -> BinResult<Self>
    where
        Self: ReadEndian,
    {
        let raw = u8::read(reader)?;
        let pos = reader.stream_position()?;

        let profile_point_detection =
            read_u8_bits::<ProfilePointDetection>(raw, MASK_PROFILE_POINT_DETECTION, 0, pos)?;
        let step_direction =
            read_u8_bits::<StepDirection>(raw, MASK_STEP_DIRECTION, SHIFT_STEP_DIRECTION, pos)?;

        Ok(Self { profile_point_detection, step_direction })
    }

    #[inline]
    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<Self> {
        Self::read(reader)
    }
}
