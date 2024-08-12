use binrw::{parser, writer, BinRead, BinResult, BinWrite, Error};
use const_format::concatcp;

pub const MIN: u16 = 0;
pub const MAX: u16 = 360;
const ERR_MESSAGE_RANGE: &'static str =
    concatcp!("train angle exceeds maximum of ", MIN, "° to ", MAX, "°");

#[inline]
pub fn valid(train_angle: u16) -> bool {
    train_angle <= MAX
}

#[parser(reader)]
pub fn parse() -> BinResult<u16> {
    let raw = u8::read(reader)?;
    let train_angle = raw as u16 * 3;

    if !valid(train_angle) {
        let pos = reader.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_MESSAGE_RANGE.to_string() });
    }

    Ok(train_angle)
}

#[writer(writer)]
pub fn write(train_angle: &u16) -> BinResult<()> {
    if !valid(*train_angle) {
        let pos = writer.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_MESSAGE_RANGE.to_string() });
    }

    let raw = (*train_angle * 3) as u8;
    raw.write(writer)?;
    Ok(())
}
