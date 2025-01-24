use binrw::{parser, writer, BinRead, BinResult, BinWrite, Error};

pub(crate) const MIN: f32 = 0.0;
pub(crate) const MAX: f32 = 3276.7;
pub(crate) const V_VALUE: f32 = 1500.0;

const MASK: u16 = 0b0111_1111_1111_1111;
const MASK_V: u16 = 0b1000_0000_0000_0000;

const ERR_MESSAGE_RANGE: &str = "sound velocity exceeds range from 0.0 to 3276.7 m/s";

#[inline]
pub fn valid(sound_velocity: f32) -> bool {
    (MIN..MAX).contains(&sound_velocity)
}

#[parser(reader, endian)]
pub fn parse() -> BinResult<f32> {
    let raw = u16::read_options(reader, endian, ())?;

    if (raw & MASK_V) != 0 {
        return Ok(1500.0);
    }

    let sound_velocity = (raw & MASK) as f32 / 10.0;
    Ok(sound_velocity)
}

#[writer(writer, endian)]
pub fn write(sound_velocity: &f32) -> BinResult<()> {
    if !valid(*sound_velocity) {
        let pos = writer.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_MESSAGE_RANGE.to_string() });
    }

    if *sound_velocity == V_VALUE {
        MASK_V.write_options(writer, endian, ())?;
        return Ok(());
    }

    let raw = (*sound_velocity * 10.0) as u16 & MASK;
    raw.write_options(writer, endian, ())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::{io::Cursor, Endian};

    use log::info;
    use test_log::test;

    #[test]
    fn test_valid() {
        let cases =
            vec![(0.0, true), (1500.0, true), (3276.7, true), (-1.0, false), (3276.8, false)];

        for &(sound_velocity, want) in cases.iter() {
            info!("Testing validity of {sound_velocity:?}, want {want:?}");
            let got = valid(sound_velocity);
            assert_eq!(want, got);
        }
    }

    const BINARY_ENDIAN: Endian = Endian::Big;
    const BINARY_CASES: [(f32, [u8; 2]); 4] = [
        (0.0, [0x00, 0x00]),    // 0.0 m/s
        (1500.0, [0x80, 0x00]), // Special value for 1500.0 m/s
        (3276.7, [0x7F, 0xFF]), // 3276.7 m/s
        (2000.0, [0x4E, 0x20]), // 2000.0 m/s
    ];

    #[test]
    fn test_parse() {
        for &(want, bytes) in BINARY_CASES.iter() {
            info!("Parsing {bytes:?}, want {want:?}");
            let mut cursor = Cursor::new(bytes);
            let got = parse(&mut cursor, BINARY_ENDIAN, ()).expect("It should not return an error");
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_write() {
        for (sound_velocity, want) in BINARY_CASES.iter() {
            info!("Writing {sound_velocity:?}, want {want:?}");
            let mut cursor = Cursor::new(Vec::new());
            write(&sound_velocity, &mut cursor, BINARY_ENDIAN, ())
                .expect("It should not return an error");
            let inner = cursor.into_inner();
            let got = inner.as_slice();
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_write_invalid() {
        let cases = [
            -1.0,   // Below minimum
            3276.8, // Above maximum
        ];

        for sound_velocity in cases.iter() {
            info!("Writing {sound_velocity:?}, want error");
            let mut cursor = Cursor::new(Vec::new());
            let error = write(sound_velocity, &mut cursor, BINARY_ENDIAN, ()).unwrap_err();
            assert!(matches!(error, Error::AssertFail { .. }));
        }
    }
}
