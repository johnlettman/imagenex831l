//! Utilities for the **Train Angle** primitive, the orientation of direction in which the sonar
//! transducer is pointed relative to the platform's fixed baseline.
//! *Ignored in Fixed-transducer sonars*.
use binrw::{parser, writer, BinRead, BinResult, BinWrite, Error};

pub(crate) const MAX: u16 = 360;

/// Validate the **Train Angle** value.
#[inline]
pub fn valid(value: u16) -> bool {
    value <= MAX
}

/// Parse the **Train Angle** from two bytes.
#[parser(reader)]
pub fn parse() -> BinResult<u16> {
    let raw = u8::read(reader)?;
    let train_angle = raw as u16 * 3;

    if !valid(train_angle) {
        let pos = reader.stream_position()?;
        return Err(Error::AssertFail {
            pos,
            message: "Train Angle exceeds range of 0 to 360°".to_string(),
        });
    }

    Ok(train_angle)
}

/// Write the **Train Angle** to two bytes.
#[writer(writer)]
pub fn write(train_angle: &u16) -> BinResult<()> {
    if !valid(*train_angle) {
        let pos = writer.stream_position()?;
        return Err(Error::AssertFail {
            pos,
            message: "Train Angle exceeds range of 0 to 360°".to_string(),
        });
    }

    let raw = (*train_angle / 3) as u8;
    raw.write(writer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ENDIAN;
    use binrw::io::Cursor;

    use log::info;
    use test_log::test;

    #[test]
    fn test_valid() {
        let cases = vec![(0, true), (180, true), (360, true), (361, false)];

        for (train_angle, want) in cases {
            info!("Testing validity of {train_angle:?}, want {want:?}");
            let got = valid(train_angle);
            assert_eq!(want, got);
        }
    }

    const BINARY_CASES: [(u16, [u8; 1]); 4] = [
        (0, [0]),     // 0° (0 * 3)
        (3, [1]),     // 3° (1 * 3)
        (180, [60]),  // 180° (60 * 3)
        (360, [120]), // 360° (120 * 3)
    ];

    #[test]
    fn test_parse() {
        for &(want, bytes) in BINARY_CASES.iter() {
            info!("Parsing {bytes:?}, want {want:?}");
            let mut cursor = Cursor::new(bytes);
            let got = parse(&mut cursor, ENDIAN, ()).expect("Should succeed");
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_parse_invalid() {
        let cases = [
            [0xC1], // 361° (out of range, 120 * 3 + 1)
            [0xFF], // 765° (out of range, 255 * 3)
        ];

        for bytes in cases.iter() {
            info!("Parsing {bytes:?}, want error");
            let mut cursor = Cursor::new(bytes);
            let error = parse(&mut cursor, ENDIAN, ()).unwrap_err();
            assert!(matches!(error, Error::AssertFail { .. }));
        }
    }

    #[test]
    fn test_write() {
        for (train_angle, want) in BINARY_CASES.iter() {
            info!("Writing {train_angle:?}, want {want:?}");
            let mut cursor = Cursor::new(Vec::new());
            write(&train_angle, &mut cursor, ENDIAN, ()).expect("It should not return an error");
            let inner = cursor.into_inner();
            let got = inner.as_slice();
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_write_invalid() {
        let cases = [
            361, // Just over max
            400, // Well over max
        ];

        for train_angle in cases.iter() {
            info!("Writing {train_angle:?}, want error");
            let mut cursor = Cursor::new(Vec::new());
            let error = write(train_angle, &mut cursor, ENDIAN, ()).unwrap_err();
            assert!(matches!(error, Error::AssertFail { .. }));
        }
    }
}
