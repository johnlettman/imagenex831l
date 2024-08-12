use binrw::{parser, writer, BinRead, BinResult, BinWrite, Error};
use const_format::concatcp;

pub const MIN: u8 = 0;
pub const MAX: u8 = 255;
const ERR_MESSAGE_RANGE: &'static str =
    concatcp!("start gain exceeds maximum of ", MIN, " to ", MAX, " dB");

#[inline]
pub fn valid(start_gain: u8) -> bool {
    MIN <= start_gain && start_gain <= MAX
}

#[parser(reader, endian)]
pub fn parse() -> BinResult<u8> {
    let start_gain = u8::read_options(reader, endian, ())?;

    if !valid(start_gain) {
        let pos = reader.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_MESSAGE_RANGE.to_string() });
    }

    Ok(start_gain)
}

#[writer(writer, endian)]
pub fn write(start_gain: &u8) -> BinResult<()> {
    if !valid(*start_gain) {
        let pos = writer.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_MESSAGE_RANGE.to_string() });
    }

    (*start_gain).write_options(writer, endian, ())?;
    Ok(())
}
