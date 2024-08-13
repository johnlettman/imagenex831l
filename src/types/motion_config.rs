use crate::types::{
    util::primitive::{read_u8_bits, write_u8_bits},
    Direction, Mode, StepSize, Transducer,
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
pub struct MotionConfig {
    pub direction: Direction,
    pub transducer: Transducer,
    pub mode: Mode,
    pub step_size: StepSize,
}

impl MotionConfig {
    const MASK_DIRECTION: u8 = 0b1000_0000;
    const MASK_TRANSDUCER: u8 = 0b0100_0000;
    const MASK_MODE: u8 = 0b0011_1000;
    const MASK_STEP_SIZE: u8 = 0b0000_0111;

    const SHIFT_DIRECTION: usize = 7;
    const SHIFT_TRANSDUCER: usize = 6;
    const SHIFT_MODE: usize = 3;
}

impl ReadEndian for MotionConfig {
    const ENDIAN: EndianKind = EndianKind::None;
}

impl WriteEndian for MotionConfig {
    const ENDIAN: EndianKind = EndianKind::None;
}

impl BinRead for MotionConfig {
    type Args<'a> = ();

    fn read<R: Read + Seek>(reader: &mut R) -> BinResult<Self>
    where
        Self: ReadEndian,
        for<'a> Self::Args<'a>: Required,
    {
        let raw = u8::read(reader)?;
        let pos = reader.stream_position()?;

        let direction =
            read_u8_bits::<Direction>(raw, Self::MASK_DIRECTION, Self::SHIFT_DIRECTION, pos)?;
        let transducer =
            read_u8_bits::<Transducer>(raw, Self::MASK_TRANSDUCER, Self::SHIFT_TRANSDUCER, pos)?;
        let mode = read_u8_bits::<Mode>(raw, Self::MASK_MODE, Self::SHIFT_MODE, pos)?;
        let step_size = read_u8_bits::<StepSize>(raw, Self::MASK_STEP_SIZE, 0, pos)?;

        Ok(MotionConfig::new(direction, transducer, mode, step_size))
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

impl BinWrite for MotionConfig {
    type Args<'a> = ();

    fn write<W: Write + Seek>(&self, writer: &mut W) -> BinResult<()>
    where
        Self: WriteEndian,
        for<'a> Self::Args<'a>: Required,
    {
        let mut raw: u8 = 0;
        let pos = writer.stream_position()?;

        raw |= write_u8_bits(self.direction, Self::MASK_DIRECTION, Self::SHIFT_DIRECTION, pos)?;
        raw |= write_u8_bits(self.transducer, Self::MASK_TRANSDUCER, Self::SHIFT_TRANSDUCER, pos)?;
        raw |= write_u8_bits(self.mode, Self::MASK_MODE, Self::SHIFT_MODE, pos)?;
        raw |= write_u8_bits(self.step_size, Self::MASK_STEP_SIZE, 0, pos)?;

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

    const BINARY_ENDIAN: Endian = Endian::NATIVE;
    const BINARY_CASES: [(MotionConfig, [u8; 1]); 3] = [
        (
            MotionConfig {
                direction: Direction::Counterclockwise,
                transducer: Transducer::Down,
                mode: Mode::Sector,
                step_size: StepSize::Slow,
            },
            [0b0000_0000],
        ),
        (
            MotionConfig {
                direction: Direction::Clockwise,
                transducer: Transducer::Up,
                mode: Mode::Polar,
                step_size: StepSize::Medium,
            },
            [0b1100_1001],
        ),
        (
            MotionConfig {
                direction: Direction::Clockwise,
                transducer: Transducer::Up,
                mode: Mode::Polar,
                step_size: StepSize::Fast,
            },
            [0b1100_1010],
        ),
    ];

    #[test]
    fn test_parse() {
        for (want, bytes) in BINARY_CASES {
            info!("Parsing {bytes:?}, want {want:?}");
            let mut cursor = Cursor::new(bytes);
            let got = MotionConfig::read(&mut cursor).expect("It should not return an error");
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_parse_options() {
        for (want, bytes) in BINARY_CASES {
            info!("Parsing {bytes:?}, want {want:?}");
            let mut cursor = Cursor::new(bytes);
            let got = MotionConfig::read_options(&mut cursor, BINARY_ENDIAN, ())
                .expect("It should not return an error");
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_write() {
        for (motion_config, want) in BINARY_CASES {
            info!("Writing {motion_config:?}, want {want:?}");
            let mut cursor = Cursor::new(Vec::new());
            motion_config.write(&mut cursor).expect("It should not return an error");
            let inner = cursor.into_inner();
            let got = inner.as_slice();
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_write_options() {
        for (motion_config, want) in BINARY_CASES {
            info!("Writing {motion_config:?}, want {want:?}");
            let mut cursor = Cursor::new(Vec::new());
            motion_config
                .write_options(&mut cursor, BINARY_ENDIAN, ())
                .expect("It should not return an error");
            let inner = cursor.into_inner();
            let got = inner.as_slice();
            assert_eq!(want, got);
        }
    }
}
