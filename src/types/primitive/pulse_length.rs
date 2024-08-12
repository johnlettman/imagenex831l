use binrw::{parser, writer, BinRead, BinResult, BinWrite, Error};
use const_format::concatcp;

pub const MAX: u16 = 1_000;
const ERR_MESSAGE_MAX_PULSE_LENGTH: &'static str =
    concatcp!("pulse length exceeds maximum of ", MAX, " μs");

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
        return Err(Error::AssertFail { pos, message: ERR_MESSAGE_MAX_PULSE_LENGTH.to_string() });
    }

    Ok(pulse_length)
}

#[writer(writer)]
pub fn write(pulse_length: &u16) -> BinResult<()> {
    if !valid(*pulse_length) {
        let pos = writer.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_MESSAGE_MAX_PULSE_LENGTH.to_string() });
    }

    let raw = (*pulse_length / 10) as u8;
    raw.write(writer)?;
    Ok(())
}
