//! Utilities for the **Sector Size** primitive. *Ignored in Fixed-transducer sonars*.
use binrw::{parser, writer, BinRead, BinResult, BinWrite, Error};
use const_format::concatcp;

pub(crate) const MAX: u16 = 360;
const ERR_RANGE: &str = concatcp!("sector size exceeds maximum of ", MAX);

#[inline]
pub fn valid(sector_size: u16) -> bool {
    sector_size <= MAX
}

#[parser(reader)]
pub fn parse() -> BinResult<u16> {
    let raw = u8::read(reader)?;
    let sector_size = raw as u16 * 3;

    if !valid(sector_size) {
        let pos = reader.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_RANGE.to_string() });
    }

    Ok(sector_size)
}

#[writer(writer)]
pub fn write(sector_size: &u16) -> BinResult<()> {
    if !valid(*sector_size) {
        let pos = writer.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_RANGE.to_string() });
    }

    let raw = (*sector_size / 3) as u8;
    raw.write(writer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::Endian;
    use std::io::Cursor;

    use log::info;
    use test_log::test;

    #[test]
    fn test_valid() {
        let cases = vec![(MAX + 1, false), (MAX - 1, true), (MAX, true)];

        for (sector_size, want) in cases {
            info!("Checking validity of {sector_size:?}, want {want:?}");
            let got = valid(sector_size);
            assert_eq!(want, got);
        }
    }

    const BINARY_ENDIAN: Endian = Endian::NATIVE;
    const BINARY_CASES: [(u16, [u8; 1]); 4] = [(360, [120]), (60, [20]), (9, [3]), (15, [5])];

    #[test]
    fn test_parse() {
        for (want, bytes) in BINARY_CASES {
            info!("Parsing {bytes:?}, want {want:?}");
            let mut cursor = Cursor::new(bytes);
            let got = parse(&mut cursor, BINARY_ENDIAN, ()).expect("It should not return an error");
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_parse_invalid() {
        let mut cursor = Cursor::new([122u8]);
        let got = parse(&mut cursor, BINARY_ENDIAN, ());
        assert!(got.is_err());
    }

    #[test]
    fn test_write() {
        for (sector_size, want) in BINARY_CASES {
            info!("Writing {sector_size:?}, want {want:?}");
            let mut cursor = Cursor::new(Vec::new());
            write(&sector_size, &mut cursor, BINARY_ENDIAN, ())
                .expect("It should not return an error");
            let inner = cursor.into_inner();
            let got = inner.as_slice();
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_write_invalid() {
        let sector_size = MAX + 1;
        let mut cursor = Cursor::new(Vec::new());
        let got = write(&sector_size, &mut cursor, BINARY_ENDIAN, ());
        assert!(got.is_err());
    }
}
