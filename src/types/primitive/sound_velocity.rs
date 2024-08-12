use binrw::{parser, writer, BinRead, BinResult, BinWrite, Error};

pub const MIN: f32 = 0.0;
pub const MAX: f32 = 3276.7;
pub const V_VALUE: f32 = 1500.0;

const MASK: u16 = 0b0111_1111_1111_1111;
const MASK_V: u16 = 0b1000_0000_0000_0000;

const ERR_MESSAGE_RANGE: &'static str = "sound velocity exceeds range from 0.0 to 3276.7 m/s";

#[inline]
pub fn valid(sound_velocity: f32) -> bool {
    MIN <= sound_velocity && sound_velocity <= MAX
}

#[parser(reader, endian)]
pub fn parse() -> BinResult<f32> {
    let raw = u16::read_options(reader, endian, ())?;

    if (raw & MASK_V) != 0 {
        return Ok(1500.0);
    }

    let sound_velocity = (raw & MASK) as f32 / 10.0;

    if !valid(sound_velocity) {
        let pos = reader.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_MESSAGE_RANGE.to_string() });
    }

    Ok(sound_velocity)
}

#[writer(writer, endian)]
pub fn write(sound_velocity: &f32) -> BinResult<()> {
    if !valid(*sound_velocity) {
        let pos = writer.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_MESSAGE_RANGE.to_string() });
    }

    if *sound_velocity == V_VALUE {
        MASK_V.write_options(writer, endian, ())?;
        return Ok(());
    }

    let raw = (*sound_velocity * 10.0) as u16 & MASK;
    raw.write_options(writer, endian, ())?;
    Ok(())
}
