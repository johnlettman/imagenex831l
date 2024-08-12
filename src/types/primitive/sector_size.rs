use binrw::{parser, writer, BinRead, BinResult, BinWrite, Error};
use const_format::concatcp;

pub const MIN: u16 = 0;
pub const MAX: u16 = 360;

const ERR_MESSAGE_RANGE: &'static str = concatcp!("sector size exceeds maximum of ", MAX);

#[inline]
pub fn valid(sector_size: u16) -> bool {
    MIN <= sector_size && sector_size <= MAX
}

#[parser(reader)]
pub fn parse() -> BinResult<u16> {
    let raw = u8::read(reader)?;
    let sector_size = raw as u16 * 3;

    if !valid(sector_size) {
        let pos = reader.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_MESSAGE_RANGE.to_string() });
    }

    Ok(sector_size)
}

#[writer(writer)]
pub fn write(sector_size: &u16) -> BinResult<()> {
    if !valid(*sector_size) {
        let pos = writer.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_MESSAGE_RANGE.to_string() });
    }

    let raw = (*sector_size / 3) as u8;
    raw.write(writer)?;
    Ok(())
}
