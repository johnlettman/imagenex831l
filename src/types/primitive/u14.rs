use binrw::{parser, writer, BinRead, BinResult, BinWrite, Error};
use const_format::concatcp;
use std::io::{Error as IOError, ErrorKind::InvalidData};

pub const MIN: u16 = 0;
pub const MAX: u16 = 0b0011_1111_1111_1111;

const MASK_HIGH: u8 = 0b0111_1110;
const MASK_LOW: u8 = 0b0111_1111;

const ERR_MESSAGE_RANGE: &'static str = concatcp!("u14 exceeds range from ", MIN, " to ", MAX);

#[inline]
pub fn valid(u14: u16) -> bool {
    MIN <= u14 && u14 <= MAX
}

#[parser(reader, endian)]
pub fn parse() -> BinResult<u16> {
    let raw = u16::read_options(reader, endian, ())?;

    // extract the high and low parts
    let high = (raw & 0xFF) as u8;
    let low = (raw >> 8) as u8;

    let high_part = (high & MASK_HIGH) >> 1;
    let low_part = (low & MASK_LOW) | ((high & 0b1) << 7);

    let value = ((high_part as u16) << 8) | (low_part as u16);
    Ok(value)
}

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

    let high_part = ((high & 0b11_1111) << 1) | (low & 0b1);
    let low_part = low >> 1;

    let raw = ((low_part as u16) << 8) | (high_part as u16);
    raw.write_options(writer, endian, ())?;
    Ok(())
}
