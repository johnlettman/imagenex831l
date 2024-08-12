use crate::types::{
    util::primitive::{read_u8_bits, write_u8_bits},
    DataBits, Logf, ProfileGrid, Zero,
};
use binrw::__private::Required;
use binrw::meta::{EndianKind, ReadEndian, WriteEndian};
use binrw::{BinRead, BinResult, BinWrite, Endian};
use std::io::{Read, Seek, Write};

#[derive(Debug, Eq, PartialEq, Clone, derive_new::new)]
#[cfg_attr(
    target_family = "wasm",
    derive(tsify::Tsify, serde::Serialize, serde::Deserialize),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct Config {
    pub profile_grid: ProfileGrid,
    pub zero: Zero,
    pub data_bits: DataBits,
    pub logf: Logf,
}

impl Config {
    const MASK_PROFILE_GRID: u8 = 0b1000_0000;
    const MASK_ZERO: u8 = 0b0100_0000;
    const MASK_DATA_BITS: u8 = 0b0011_1000;
    const MASK_LOGF: u8 = 0b0000_0111;

    const SHIFT_PROFILE_GRID: usize = 7;
    const SHIFT_ZERO: usize = 6;
    const SHIFT_DATA_BITS: usize = 3;
}

impl ReadEndian for Config {
    const ENDIAN: EndianKind = EndianKind::None;
}

impl WriteEndian for Config {
    const ENDIAN: EndianKind = EndianKind::None;
}

impl BinRead for Config {
    type Args<'a> = ();

    fn read<R: Read + Seek>(reader: &mut R) -> BinResult<Self>
    where
        Self: ReadEndian,
        for<'a> Self::Args<'a>: Required,
    {
        let raw = u8::read(reader)?;
        let pos = reader.stream_position()?;

        let profile_grid = read_u8_bits::<ProfileGrid>(
            raw,
            Self::MASK_PROFILE_GRID,
            Self::SHIFT_PROFILE_GRID,
            pos,
        )?;
        let zero = read_u8_bits::<Zero>(raw, Self::MASK_ZERO, Self::SHIFT_ZERO, pos)?;
        let data_bits =
            read_u8_bits::<DataBits>(raw, Self::MASK_DATA_BITS, Self::SHIFT_DATA_BITS, pos)?;
        let logf = read_u8_bits::<Logf>(raw, Self::MASK_LOGF, 0, pos)?;

        Ok(Config::new(profile_grid, zero, data_bits, logf))
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

impl BinWrite for Config {
    type Args<'a> = ();

    fn write<W: Write + Seek>(&self, writer: &mut W) -> BinResult<()>
    where
        Self: WriteEndian,
        for<'a> Self::Args<'a>: Required,
    {
        let mut raw: u8 = 0;
        let pos = writer.stream_position()?;

        raw |= write_u8_bits(
            self.profile_grid,
            Self::MASK_PROFILE_GRID,
            Self::SHIFT_PROFILE_GRID,
            pos,
        )?;
        raw |= write_u8_bits(self.zero, Self::MASK_ZERO, Self::SHIFT_ZERO, pos)?;
        raw |= write_u8_bits(self.data_bits, Self::MASK_DATA_BITS, Self::SHIFT_DATA_BITS, pos)?;
        raw |= write_u8_bits(self.logf, Self::MASK_LOGF, 0, pos)?;

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
