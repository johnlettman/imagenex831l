use binrw::{parser, writer, BinRead, BinResult, BinWrite, Error};
use const_format::concatcp;
use std::io::{Error as IOError, ErrorKind::InvalidData};

const MAX: i16 = 8191;
const MIN: i16 = -8192;

const FLAG_1: u8 = 0b1000_0000;
const FLAG_2: u8 = 0b0100_0000;

const MASK_I14: u16 = 0b0011_1111_1111_1111;
const MASK_I14_HIGH: u8 = 0b0011_1111;
const MASK_I14_LOW: u8 = 0b1111_1111;

const SIGN_I14: u16 = 0b0010_0000_0000_0000;
const SIGN_I16_FILL: u16 = 0b1100_0000_0000_0000;

const ERR_MESSAGE_RANGE: &'static str =
    concatcp!("Invalid i14, exceeds range from ", MIN, " to ", MAX);

pub fn valid_i14(i14: i16) -> bool {
    MIN <= i14 && i14 <= MAX
}

#[parser(reader, endian)]
pub fn parse() -> BinResult<(i16, bool, bool)> {
    let raw = u16::read_options(reader, endian, ())?;

    // extract the high and low parts
    let low = (raw >> 8) as u8;
    let high = (raw & MASK_I14_LOW as u16) as u8;

    // extract the flags
    let flag1 = (high & FLAG_1) != 0;
    let flag2 = (high & FLAG_2) != 0;

    // assemble the integer
    let high_part = (high & MASK_I14_HIGH) as u16;
    let assembled = ((high_part << 8) | (low as u16)) & MASK_I14;

    // convert the assembled integer into a 14-bit two's compliment signed integer
    let i14 = if assembled & SIGN_I14 != 0 {
        // extend the 14-bit sign
        assembled | SIGN_I16_FILL
    } else {
        assembled
    } as i16;

    if !valid_i14(i14) {
        let pos = reader.stream_position()?;
        let message = format!("{} ({})", ERR_MESSAGE_RANGE, i14);
        return Err(Error::Custom { pos, err: Box::new(IOError::new(InvalidData, message)) });
    }

    Ok((i14, flag1, flag2))
}

#[writer(writer, endian)]
pub fn write(values: &(i16, bool, bool)) -> BinResult<()> {
    let (i14, flag1, flag2) = *values;

    if !valid_i14(i14) {
        let pos = writer.stream_position()?;
        let message = format!("{} ({})", ERR_MESSAGE_RANGE, i14);
        return Err(Error::Custom { pos, err: Box::new(IOError::new(InvalidData, message)) });
    }

    let mut high = (i14 as u16 >> 8) as u8 & MASK_I14_HIGH;
    let low = (i14 as u16 & MASK_I14_LOW as u16) as u8;

    if flag1 {
        high |= FLAG_1;
    }

    if flag2 {
        high |= FLAG_2;
    }

    let raw = ((low as u16) << 8) | (high as u16);
    raw.write_options(writer, endian, ())
}

#[cfg(test)]
mod tests {
    use super::*;

    use binrw::Endian;
    use log::info;
    use std::io::Cursor;
    use test_log::test;

    const BINARY_ENDIAN: Endian = Endian::Big;
    const BINARY_CASES: [(i16, bool, bool, [u8; 2]); 9] = [
        (0, false, false, [0x00, 0x00]),
        (1, false, false, [0b0000_0001, 0b0000_0000]),
        (2, true, true, [0b0000_0010, 0b1100_0000]),
        (3, true, false, [0b0000_0011, 0b1000_0000]),
        (4, false, true, [0b0000_0100, 0b0100_0000]),
        (-1, false, false, [0b1111_1111, 0b0011_1111]),
        (8191, false, false, [0b1111_1111, 0b0001_1111]),
        (-8192, false, false, [0b0000_0000, 0b0010_0000]),
        (1234, true, false, [0b1101_0010, 0b1000_0100]),
    ];

    #[test]
    fn test_valid_i14() {
        let cases = vec![
            (0, true),
            (1, true),
            (-1, true),
            (1000, true),
            (-1000, true),
            (MAX + 1, false),
            (MIN - 1, false),
        ];

        for (i14, want) in cases {
            info!("Testing {i14:?}, expecting {want:?}");
            let got = valid_i14(i14);
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_parse() {
        for &(want_i14, want_flag1, want_flag2, ref bytes) in BINARY_CASES.iter() {
            let want = (want_i14, want_flag1, want_flag2);

            info!("Parsing {bytes:?}, expecting {want:?}");
            let mut cursor = Cursor::new(bytes);
            let got = parse(&mut cursor, BINARY_ENDIAN, ()).expect("Should not return an error");

            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_write() {
        for &(i14, flag1, flag2, ref want) in BINARY_CASES.iter() {
            let values = (i14, flag1, flag2);

            info!("Writing {values:?}, expecting {want:?}");
            let mut cursor = Cursor::new(Vec::new());
            write(&(i14, flag1, flag2), &mut cursor, BINARY_ENDIAN, ())
                .expect("Should not return an error");

            let got_inner = cursor.into_inner();
            let got = got_inner.as_slice();
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_write_invalid() {
        let invalid_i14 = MAX + 1;
        let values = (invalid_i14, true, false);

        let mut cursor = Cursor::new(Vec::new());
        let got = write(&values, &mut cursor, BINARY_ENDIAN, ());
        assert!(got.is_err());
    }
}
