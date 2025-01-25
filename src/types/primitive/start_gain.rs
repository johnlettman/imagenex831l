//! Utilities for the **Start Gain** primitive, the initial amplification applied to the received
//! echo signal at the beginning of its travel through the sonar receiver.
use crate::ENDIAN;
use binrw::{parser, writer, BinRead, BinResult, BinWrite, Error};

pub(crate) const MAX: u8 = 40;

/// Validate if the provided **Start Gain** fits in a byte.
#[inline]
pub fn valid(value: u8) -> bool {
    value <= MAX
}

/// Parse **Start Gain** from a byte.
#[parser(reader)]
pub fn parse() -> BinResult<u8> {
    let start_gain = u8::read_options(reader, ENDIAN, ())?;

    if !valid(start_gain) {
        let pos = reader.stream_position()?;
        return Err(Error::AssertFail {
            pos,
            message: "Start gain exceeds range of 0 to 40 dB".to_string(),
        });
    }

    Ok(start_gain)
}

/// Write **Start Gain** to a byte.
#[writer(writer)]
pub fn write(start_gain: &u8) -> BinResult<()> {
    if !valid(*start_gain) {
        let pos = writer.stream_position()?;
        return Err(Error::AssertFail {
            pos,
            message: "Start gain exceeds range of 0 to 40 dB".to_string(),
        });
    }

    (*start_gain).write_options(writer, ENDIAN, ())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    use log::info;
    use test_log::test;

    const BINARY_CASES: [(u8, [u8; 1]); 2] = [(12, [12]), (30, [30])];

    #[test]
    fn test_valid() {
        let cases =
            vec![(0, true), (20, true), (40, true), (41, false), (200, false), (255, false)];

        for (start_gain, want) in cases {
            info!("Testing validity of {start_gain:?}, want {want:?}");
            let got = valid(start_gain);
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_parse() {
        for (want, bytes) in BINARY_CASES {
            info!("Parsing {bytes:?}, expecting {want:?}");
            let mut cursor = Cursor::new(bytes);
            let got = parse(&mut cursor, ENDIAN, ()).expect("Should succeed");
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_write() {
        for (start_gain, want) in BINARY_CASES {
            info!("Writing {start_gain:?}, want {want:?}");
            let mut cursor = Cursor::new(Vec::new());
            write(&start_gain, &mut cursor, ENDIAN, ()).expect("Should succeed");
            let inner = cursor.into_inner();
            let got = inner.as_slice();
            assert_eq!(want, got);
        }
    }
}
