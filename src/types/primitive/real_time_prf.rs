use binrw::{parser, writer, BinRead, BinResult, BinWrite, Error};

pub const MIN: f32 = 0.0;
pub const MAX: f32 = 327.67;

const MASK: u16 = 0b0111_1111_1111_1111;

const ERR_MESSAGE_RANGE: &'static str = "real-time pRF exceeds range of 0.0 to 327.67 Hz";

pub fn valid(real_time_prf: f32) -> bool {
    MIN <= real_time_prf && real_time_prf <= MAX
}

#[parser(reader, endian)]
pub fn parse() -> BinResult<f32> {
    let raw = u16::read_options(reader, endian, ())?;
    let real_time_prf = (raw & MASK) as f32 / 100.0;

    if !valid(real_time_prf) {
        let pos = reader.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_MESSAGE_RANGE.to_string() });
    }

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
