use binrw::{parser, writer, BinRead, BinResult, BinWrite, Error};

pub(crate) const MIN: f32 = 0.0;
pub(crate) const MAX: f32 = 327.67;

const MASK: u16 = 0b0111_1111_1111_1111;

const ERR_MESSAGE_RANGE: &str = "real-time pRF exceeds range of 0.0 to 327.67 Hz";

pub fn valid(real_time_prf: f32) -> bool {
    (MIN..MAX).contains(&real_time_prf)
}

#[parser(reader, endian)]
pub fn parse() -> BinResult<f32> {
    let raw = u16::read_options(reader, endian, ())?;
    let real_time_prf = (raw & MASK) as f32 / 100.0;
    Ok(real_time_prf)
}

#[writer(writer, endian)]
pub fn write(real_time_prf: &f32) -> BinResult<()> {
    if !valid(*real_time_prf) {
        let pos = writer.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_MESSAGE_RANGE.to_string() });
    }

    let raw = (*real_time_prf * 100.0).round() as u16 & MASK;
    raw.write_options(writer, endian, ())
}

#[cfg(test)]
mod tests {
    use super::*;

    use log::info;
    use test_log::test;

    use binrw::{io::Cursor, Endian};

    #[test]
    fn test_valid() {
        let cases =
            vec![(0.0, true), (327.67, true), (150.25, true), (-1.0, false), (327.68, false)];

        for (prf, want) in cases {
            info!("Testing validity of {prf:?}, expecting {want:?}");
            let got = valid(prf);
            assert_eq!(want, got);
        }
    }

    const BINARY_ENDIAN: Endian = Endian::Big;
    const BINARY_CASES: [(f32, [u8; 2]); 4] = [
        (0.00, [0x00, 0x00]),   // 0.00 Hz
        (4.00, [0x01, 0x90]),   // 4.00 Hz
        (327.67, [0x7F, 0xFF]), // 327.67 Hz
        (200.00, [0x4E, 0x20]), // 200.00 Hz
    ];

    #[test]
    fn test_parse() {
        for &(want, bytes) in BINARY_CASES.iter() {
            let mut cursor = Cursor::new(bytes);
            let result =
                parse(&mut cursor, BINARY_ENDIAN, ()).expect("It should not return an error");
            assert_eq!(result, want);
        }
    }

    #[test]
    fn test_write() {
        for (prf, want) in BINARY_CASES.iter() {
            let mut buffer = Cursor::new(Vec::new());
            write(prf, &mut buffer, BINARY_ENDIAN, ()).expect("It should not return an error");
            let inner = buffer.into_inner();
            let got = inner.as_slice();
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_write_invalid() {
        let cases = [
            -1.0,   // Below min value
            327.68, // Above max value
        ];

        for prf in cases.iter() {
            let mut buffer = Cursor::new(Vec::new());
            let error = write(prf, &mut buffer, BINARY_ENDIAN, ()).unwrap_err();
            assert!(matches!(error, Error::AssertFail { .. }));
        }
    }
}
