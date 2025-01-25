//! Utilities for the `u14` primitive, an unsigned 14-bit integer.
//!
//! This primitive is used for **Profile Range** and **Data Bytes**.
//!
//! ## Wire format
//!
//! <table>
//! <tr>
//! <th colspan="8">Low Byte</th>
//! <th colspan="8">High Byte</th>
//! </tr>
//! <tr>
//!     <td><code>7</code></td><td><code>6</code></td><td><code>5</code></td><td><code>4</code></td>
//!     <td><code>3</code></td><td><code>2</code></td><td><code>1</code></td><td><code>0</code></td>
//!     <td><code>7</code></td><td><code>6</code></td><td><code>5</code></td><td><code>4</code></td>
//!     <td><code>3</code></td><td><code>2</code></td><td><code>1</code></td><td><code>0</code></td>
//! </tr>
//! <tr>
//!     <td><em>0</em></td><td colspan="7"><em>u14 LOW</em></td>
//!     <td><em>0</em></td><td colspan="6"><em>u14 HIGH</em></td>
//!     <td><em>u14 LOW BIT 0</em></td>
//! </tr>
//! </table>
use binrw::{parser, writer, BinRead, BinResult, BinWrite, Error};
use const_format::concatcp;
use std::io::{Error as IOError, ErrorKind::InvalidData};

pub(crate) const MAX: u16 = 0b0011_1111_1111_1111;

const MASK_HIGH: u8 = 0b0111_1110;
const SHIFT_HIGH: usize = 1;

const MASK_HIGH_L: u8 = 0b0000_0001;
const SHIFT_HIGH_L: usize = 7;

const MASK_LOW: u8 = 0b0111_1111;

const ERR_MESSAGE_RANGE: &str = concatcp!("u14 exceeds maximum of ", MAX);

/// Validate if the provided value can fit in a `u14`.
#[inline]
pub fn valid(value: u16) -> bool {
    value <= MAX
}

/// Parse an 831L-formatted `u14` from two bytes.
#[parser(reader, endian)]
pub fn parse() -> BinResult<u16> {
    let raw = u16::read_options(reader, endian, ())?;

    // extract the high and low parts
    let high = (raw & 0xFF) as u8;
    let low = (raw >> 8) as u8;

    let high_part = (high & MASK_HIGH) >> SHIFT_HIGH;
    let low_part = (low & MASK_LOW) | ((high & MASK_HIGH_L) << SHIFT_HIGH_L);

    let value = ((high_part as u16) << 8) | (low_part as u16);
    Ok(value)
}

/// Write an 831L-formatted `u14` to two bytes.
#[writer(writer, endian)]
pub fn write(u14: &u16) -> BinResult<()> {
    if !valid(*u14) {
        let pos = writer.stream_position()?;
        return Err(Error::Custom {
            pos,
            err: Box::new(IOError::new(InvalidData, ERR_MESSAGE_RANGE)),
        });
    }

    let high = (*u14 >> 8) as u8;
    let low = (*u14 & 0xFF) as u8;

    let high_part = ((high << SHIFT_HIGH) & MASK_HIGH) | ((low >> SHIFT_HIGH_L) & MASK_HIGH_L);
    let low_part = low & MASK_LOW;

    let raw = ((low_part as u16) << 8) | (high_part as u16);
    raw.write_options(writer, endian, ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::{io::Cursor, BinRead, BinWrite, Endian};

    use log::info;
    use test_log::test;

    #[test]
    fn test_valid() {
        let cases = vec![(0, true), (8191, true), (16383, true), (16384, false), (65535, false)];

        for (u14_value, want) in cases {
            info!("Testing validity of {u14_value:?}, want {want:?}");
            let got = valid(u14_value);
            assert_eq!(want, got);
        }
    }

    const BINARY_ENDIAN: Endian = Endian::Big;
    const BINARY_CASES: [(u16, [u8; 2]); 3] = [
        (0x0000, [0x00, 0x00]),
        (0b0011_1111_1111_1111, [0b0111_1111, 0b0111_1111]),
        (0b0010_1010_1010_1010, [0b0010_1010, 0b0101_0101]),
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
        for (u14_value, want) in BINARY_CASES.iter() {
            info!("Writing {u14_value:?}, want {want:?}");
            let mut cursor = Cursor::new(Vec::new());
            write(&u14_value, &mut cursor, BINARY_ENDIAN, ())
                .expect("It should not return an error");
            let inner = cursor.into_inner();
            let got = inner.as_slice();
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_write_invalid() {
        let cases = [
            16384, // Invalid: just over the maximum for u14
            32768, // Invalid: exceeds the u14 range
        ];

        for u14_value in cases.iter() {
            info!("Writing {u14_value:?}, want error");
            let mut cursor = Cursor::new(Vec::new());
            let error = write(u14_value, &mut cursor, BINARY_ENDIAN, ()).unwrap_err();
            assert!(matches!(error, Error::Custom { .. }));
        }
    }
}
