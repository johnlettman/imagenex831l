use crate::types::{primitive::u14, RangeIndex};
use binrw::{parser, writer, BinResult, BinWrite};

const SCALE: f32 = 0.0005;

#[parser(reader, endian)]
pub fn parse(range_index: RangeIndex) -> BinResult<f32> {
    let value = u14::parse(reader, endian, ())?;
    let profile_range = (value as f32 * SCALE) + range_index.filter_delay();
    Ok(profile_range)
}

#[writer(writer, endian)]
pub fn write(profile_range: &f32, range_index: &RangeIndex) -> BinResult<()> {
    let value = ((*profile_range / SCALE) - range_index.filter_delay()) as u16;
    value.write_options(writer, endian, ())?;
    Ok(())
}
