use crate::types::{
    util::primitive::{read_u8_bits, write_u8_bits},
    DataBits, Logf, ProfileGrid, Zero,
};
use binrw::meta::{EndianKind, ReadEndian, WriteEndian};
use binrw::{BinRead, BinResult, BinWrite, Endian};
use std::io::{Read, Seek, Write};

const MASK_PROFILE_GRID: u8 = 0b1000_0000;
const MASK_ZERO: u8 = 0b0100_0000;
const MASK_DATA_BITS: u8 = 0b0011_1000;
const MASK_LOGF: u8 = 0b0000_0111;

const SHIFT_PROFILE_GRID: usize = 7;
const SHIFT_ZERO: usize = 6;
const SHIFT_DATA_BITS: usize = 3;

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
    {
        let raw = u8::read(reader)?;
        let pos = reader.stream_position()?;

        let profile_grid =
            read_u8_bits::<ProfileGrid>(raw, MASK_PROFILE_GRID, SHIFT_PROFILE_GRID, pos)?;
        let zero = read_u8_bits::<Zero>(raw, MASK_ZERO, SHIFT_ZERO, pos)?;
        let data_bits = read_u8_bits::<DataBits>(raw, MASK_DATA_BITS, SHIFT_DATA_BITS, pos)?;
        let logf = read_u8_bits::<Logf>(raw, MASK_LOGF, 0, pos)?;

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
    {
        let mut raw: u8 = 0;
        let pos = writer.stream_position()?;

        raw |= write_u8_bits(self.profile_grid, MASK_PROFILE_GRID, SHIFT_PROFILE_GRID, pos)?;
        raw |= write_u8_bits(self.zero, MASK_ZERO, SHIFT_ZERO, pos)?;
        raw |= write_u8_bits(self.data_bits, MASK_DATA_BITS, SHIFT_DATA_BITS, pos)?;
        raw |= write_u8_bits(self.logf, MASK_LOGF, 0, pos)?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    use log::info;
    use test_log::test;

    const BINARY_ENDIAN: Endian = Endian::Big;
    const BINARY_CASES: [(Config, [u8; 1]); 3] = [
        (
            Config {
                profile_grid: ProfileGrid::Off,
                zero: Zero::Up,
                data_bits: DataBits::X4Bits,
                logf: Logf::X10dB,
            },
            [0b0000_0000],
        ),
        (
            Config {
                profile_grid: ProfileGrid::On,
                zero: Zero::Down,
                data_bits: DataBits::X8Bits,
                logf: Logf::X40dB,
            },
            [0b1100_1011],
        ),
        (
            Config {
                profile_grid: ProfileGrid::On,
                zero: Zero::Up,
                data_bits: DataBits::X14Bits,
                logf: Logf::X40dB,
            },
            [0b1001_0011],
        ),
    ];

    #[test]
    fn test_parse() {
        for (want, bytes) in BINARY_CASES {
            info!("Parsing {bytes:?}, want {want:?}");
            let mut cursor = Cursor::new(bytes);
            let got = Config::read_options(&mut cursor, BINARY_ENDIAN, ())
                .expect("It should not return an error");
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_write() {
        for (config, want) in BINARY_CASES {
            info!("Writing {config:?}, want {want:?}");
            let mut cursor = Cursor::new(Vec::new());
            config
                .write_options(&mut cursor, BINARY_ENDIAN, ())
                .expect("It should not return an error");
            let inner = cursor.into_inner();
            let got = inner.as_slice();
            assert_eq!(want, got);
        }
    }
}
