use binrw::{parser, BinRead, BinResult, Error};

pub const MIN: f32 = 0.0;
pub const MAX: f32 = 250.0;

const ERR_MESSAGE_RANGE: &'static str = "profile minimum range exceeds range from 0.0m to 250.0m";

#[inline]
pub fn valid(profile_min_range: f32) -> bool {
    MIN <= profile_min_range && profile_min_range <= MAX
}

#[parser(reader)]
pub fn parse() -> BinResult<f32> {
    let profile_min_range = u8::read(reader)? as f32;

    if !valid(profile_min_range) {
        let pos = reader.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_MESSAGE_RANGE.to_string() });
    }

    Ok(profile_min_range)
}
