use crate::types::util::primitive::{read_u8_bits, write_u8_bits};
use crate::types::{ProfilePointDetection, StepDirection};
use binrw::meta::{EndianKind, ReadEndian, WriteEndian};
use binrw::{BinRead, BinResult, BinWrite, Endian};
use std::io::{Read, Seek, Write};

const MASK_STEP_DIRECTION: u8 = 0b0100_0000;
const SHIFT_STEP_DIRECTION: usize = 6;

const MASK_PROFILE_POINT_DETECTION: u8 = 0b0000_0001;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, derive_new::new)]
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
pub struct Command {
    #[cfg(not(feature = "pyo3"))]
    pub profile_point_detection: ProfilePointDetection,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    pub profile_point_detection: ProfilePointDetection,

    #[cfg(not(feature = "pyo3"))]
    pub step_direction: StepDirection,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
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

impl BinWrite for Command {
    type Args<'a> = ();

    fn write<W: Write + Seek>(&self, writer: &mut W) -> BinResult<()>
    where
        Self: WriteEndian,
    {
        let mut raw: u8 = 0;
        let pos = writer.stream_position()?;

        raw |= write_u8_bits(self.profile_point_detection, MASK_PROFILE_POINT_DETECTION, 0, pos)?;
        raw |= write_u8_bits(self.step_direction, MASK_STEP_DIRECTION, SHIFT_STEP_DIRECTION, pos)?;

        raw.write(writer)
    }

    #[inline]
    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        _: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<()> {
        self.write(writer)
    }
}
