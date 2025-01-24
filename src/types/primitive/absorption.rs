use binrw::{parser, writer, BinRead, BinResult, BinWrite, Error};

pub(crate) const MIN: f32 = 0.0;
pub(crate) const MAX: f32 = 2.55;
const ERR_MESSAGE_RANGE: &str = "absorption exceeds range from 0.0 to 2.55 dB/m";

#[inline]
pub fn valid(absorption: f32) -> bool {
    (MIN..MAX).contains(&absorption)
}

#[parser(reader)]
pub fn parse() -> BinResult<f32> {
    let raw = u8::read(reader)?;
    let absorption = raw as f32 / 100.0;

    if !valid(absorption) {
        let pos = reader.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_MESSAGE_RANGE.to_string() });
    }

    Ok(absorption)
}

#[writer(writer)]
pub fn write(absorption: &f32) -> BinResult<()> {
    if !valid(*absorption) {
        let pos = writer.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_MESSAGE_RANGE.to_string() });
    }

    let raw = (*absorption * 100.0).round() as u8;
    raw.write(writer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use binrw::Endian;
    use log::info;
    use std::io::Cursor;
    use test_log::test;

    const BINARY_ENDIAN: Endian = Endian::Big;
    const BINARY_CASES: [(f32, [u8; 1]); 3] = [(1.0, [100u8]), (2.55, [255u8]), (0.0, [0u8])];

    const INVALID_CASES: [f32; 4] = [MIN - 0.1, MIN - 1.0, MAX + 0.1, MAX + 1.0];

    #[test]
    fn test_valid() {
        let cases =
            vec![(0.0, true), (0.0, true), (0.0, true), (MIN - 0.1, false), (MAX + 0.1, false)];

        for (absorption, want) in cases {
            info!("Validating {absorption:?}, expecting {want:?}");
            let got = valid(absorption);
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_parse() {
        for &(want, ref bytes) in BINARY_CASES.iter() {
            info!("Parsing {bytes:?}, expecting {want:?}");
            let mut cursor = Cursor::new(bytes);
            let got = parse(&mut cursor, BINARY_ENDIAN, ()).expect("Should not return an error");

            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_write() {
        for &(absorption, ref want) in BINARY_CASES.iter() {
            info!("Writing {absorption:?}, expecting {want:?}");
            let mut cursor = Cursor::new(Vec::new());
            write(&absorption, &mut cursor, BINARY_ENDIAN, ()).expect("Should not return an error");

            let got_inner = cursor.into_inner();
            let got = got_inner.as_slice();

            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_write_invalid() {
        for absorption in INVALID_CASES {
            let mut cursor = Cursor::new(Vec::new());
            let got = write(&absorption, &mut cursor, BINARY_ENDIAN, ());
            assert!(got.is_err(), "Should return an error");
        }
    }
}
