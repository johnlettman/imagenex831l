use binrw::{parser, writer, BinRead, BinResult, BinWrite, Error};

pub(crate) const MIN: f32 = 0.0;
pub(crate) const MAX: f32 = 250.0;

const ERR_MESSAGE_RANGE: &str = "profile minimum range exceeds range from 0.0m to 250.0m";

#[inline]
pub fn valid(profile_min_range: f32) -> bool {
    (MIN..=MAX).contains(&profile_min_range)
}

#[parser(reader)]
pub fn parse() -> BinResult<f32> {
    let profile_min_range = u8::read(reader)? as f32;

    if !valid(profile_min_range) {
        let pos = reader.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_MESSAGE_RANGE.to_string() });
    }

    Ok(profile_min_range)
}

#[writer(writer)]
pub fn write(profile_min_range: &f32) -> BinResult<()> {
    if !valid(*profile_min_range) {
        let pos = writer.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_MESSAGE_RANGE.to_string() });
    }

    (*profile_min_range as u8).write(writer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::{io::Cursor, BinRead, BinWrite, Endian};

    use log::info;
    use test_log::test;

    #[test]
    fn test_valid() {
        let cases = vec![(0.0, true), (125.0, true), (250.0, true), (-1.0, false), (251.0, false)];

        for (profile_min_range, want) in cases {
            info!("Testing validity of {profile_min_range:?}, want {want:?}");
            let got = valid(profile_min_range);
            assert_eq!(want, got);
        }
    }

    const BINARY_ENDIAN: Endian = Endian::NATIVE;
    const BINARY_CASES: [(f32, [u8; 1]); 4] = [
        (0.0, [0x00]),   // 0.0m
        (125.0, [0x7D]), // 125.0m
        (250.0, [0xFA]), // 250.0m
        (100.0, [0x64]), // 100.0m
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
    fn test_parse_invalid() {
        let cases = [
            [0xFB], // 251.0m (out of range)
            [0xFF], // 255.0m (out of range)
        ];

        for bytes in cases.iter() {
            info!("Parsing {bytes:?}, want error");
            let mut cursor = Cursor::new(bytes);
            let error = parse(&mut cursor, BINARY_ENDIAN, ()).unwrap_err();
            assert!(matches!(error, Error::AssertFail { .. }));
        }
    }

    #[test]
    fn test_write() {
        for (profile_min_range, want) in BINARY_CASES.iter() {
            info!("Writing {profile_min_range:?}, want {want:?}");
            let mut cursor = Cursor::new(Vec::new());
            write(&profile_min_range, &mut cursor, BINARY_ENDIAN, ())
                .expect("It should not return an error");
            let inner = cursor.into_inner();
            let got = inner.as_slice();
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_write_invalid() {
        let cases = [
            -1.0,  // Below minimum
            251.0, // Above maximum
        ];

        for profile_min_range in cases.iter() {
            info!("Writing {profile_min_range:?}, want error");
            let mut cursor = Cursor::new(Vec::new());
            let error = write(profile_min_range, &mut cursor, BINARY_ENDIAN, ()).unwrap_err();
            assert!(matches!(error, Error::AssertFail { .. }));
        }
    }
}
