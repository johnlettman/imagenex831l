use crate::types::{
    primitive::{profile_range, u14},
    Acceleration, Angle, HeadPosition, RangeIndex, SonarReturnMagic, SonarReturnStatus, SonarType,
};
use binrw::{BinRead, BinWrite};

#[derive(Debug, BinRead, BinWrite, Clone)]
pub struct SonarReturnHeader {
    pub magic: SonarReturnMagic,
    pub sonar_type: SonarType,
    pub status: SonarReturnStatus,
    pub head_position: HeadPosition,
    pub range_index: RangeIndex,

    #[br(parse_with = profile_range::parse)]
    #[bw(write_with = profile_range::write)]
    #[br(args(range_index))]
    #[bw(args(range_index))]
    pub profile_range: f32,

    #[br(parse_with = u14::parse)]
    #[bw(write_with = u14::write)]
    #[brw(pad_after = 4)]
    pub data_length: u16,

    pub roll_angle: Angle,
    pub pitch_angle: Angle,
    pub roll_acceleration: Acceleration,
    #[brw(pad_after = 8)]
    pub pitch_acceleration: Acceleration,
}
