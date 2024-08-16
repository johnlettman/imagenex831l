use binrw::{parser, writer, BinRead, BinResult, BinWrite, Error};
use const_format::concatcp;

pub const MAX: u16 = 1_000;
const ERR_MESSAGE_RANGE: &'static str = concatcp!("pulse length exceeds maximum of ", MAX, " μs");

#[inline]
pub fn valid(pulse_length: u16) -> bool {
    pulse_length <= MAX
}

#[parser(reader)]
pub fn parse() -> BinResult<u16> {
    let raw = u8::read(reader)?;
    let pulse_length = raw as u16 * 10;

    if !valid(pulse_length) {
        let pos = reader.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_MESSAGE_RANGE.to_string() });
    }

    Ok(pulse_length)
}

#[writer(writer)]
pub fn write(pulse_length: &u16) -> BinResult<()> {
    if !valid(*pulse_length) {
        let pos = writer.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_MESSAGE_RANGE.to_string() });
    }

    let raw = (*pulse_length / 10) as u8;
    raw.write(writer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::{io::Cursor, Endian};
    use log::info;

    #[test]
    fn test_valid() {
        assert!(valid(0));
        assert!(valid(500));
        assert!(valid(1_000));
        assert!(!valid(1_001));
    }

    const BINARY_ENDIAN: Endian = Endian::Big;
    const BINARY_CASES: [(u16, [u8; 1]); 3] = [
        (0, [0x00]),     // 0 μs
        (500, [0x32]),   // 500 μs
        (1_000, [0x64]), // 1,000 μs
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
            [0x65], // 1,010 μs (out of range)
            [0xFF], // 2,550 μs (out of range)
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
        for (pulse_length, want) in BINARY_CASES.iter() {
            info!("Writing {pulse_length:?}, want {want:?}");
            let mut cursor = Cursor::new(Vec::new());
            write(pulse_length, &mut cursor, BINARY_ENDIAN, ())
                .expect("It should not return an error");
            let inner = cursor.into_inner();
            let got = inner.as_slice();
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_write_invalid() {
        let cases = [
            1_010, // Just over max
            2_550, // Well over max
        ];

        for pulse_length in cases.iter() {
            info!("Writing {pulse_length:?}, want error");
            let mut cursor = Cursor::new(Vec::new());
            let error = write(pulse_length, &mut cursor, BINARY_ENDIAN, ()).unwrap_err();
            assert!(matches!(error, Error::AssertFail { .. }));
        }
    }
}
