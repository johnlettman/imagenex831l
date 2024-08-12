use crate::types::{
    primitive::{
        absorption, datetime, pulse_length, real_time_prf, sector_size, sound_velocity, start_gain,
        train_angle,
    },
    Config, DataSizeIndex, MotionConfig, RangeIndex, SensorAvailable, SensorInformation,
};
use binrw::{BinRead, BinWrite};
use chrono::{DateTime, Utc};

#[derive(Debug, BinRead, BinWrite, PartialEq, Clone)]
#[brw(big)]
#[br(
    assert(total_length == Self::VALID_TOTAL_LENGTH),
    assert(data_length == Self::VALID_DATA_LENGTH)
)]
#[bw(
    assert(* total_length == Self::VALID_TOTAL_LENGTH),
    assert(* data_length == Self::VALID_DATA_LENGTH)
)]
pub struct FileHeader {
    data_size_index: DataSizeIndex,
    total_length: u16,
    data_length: u16,

    #[br(parse_with = datetime::parse)]
    #[bw(write_with = datetime::write)]
    #[brw(pad_after = 1)]
    datetime: DateTime<Utc>,

    #[brw(pad_after = 2)]
    sensor_available: SensorAvailable,

    motion: MotionConfig,

    #[br(parse_with = start_gain::parse)]
    #[bw(write_with = start_gain::write)]
    start_gain: u8,

    #[br(parse_with = sector_size::parse)]
    #[bw(write_with = sector_size::write)]
    sector_size: u16,

    #[br(parse_with = train_angle::parse)]
    #[bw(write_with = train_angle::write)]
    train_angle: u16,

    range_code: RangeIndex,

    #[br(parse_with = absorption::parse)]
    #[bw(write_with = absorption::write)]
    absorption: f32,

    config: Config,

    #[br(parse_with = pulse_length::parse)]
    #[bw(write_with = pulse_length::write)]
    #[brw(pad_after = 1)]
    pulse_length: u16,

    #[br(parse_with = sound_velocity::parse)]
    #[bw(write_with = sound_velocity::write)]
    #[brw(pad_after = 31)]
    sound_velocity: f32,

    operating_frequency: u16,

    #[br(parse_with = real_time_prf::parse)]
    #[bw(write_with = real_time_prf::write)]
    #[brw(pad_after = 15)]
    real_time_prf: f32,

    sensor_information: SensorInformation,

    pitch: f32,
    roll: f32,
    distance: f32,
}

impl FileHeader {
    const VALID_TOTAL_LENGTH: u16 = 512;
    const VALID_DATA_LENGTH: u16 = 283;
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::types::{DataBits, Direction, Logf, Mode, ProfileGrid, StepSize, Transducer, Zero};
    use std::io::Cursor;
    use test_log::test;

    #[test]
    fn test_parse() {
        let sample_bytes: [u8; 108] = [
            0x02, 0x02, 0x00, 0x01, 0x1b, 0x32, 0x37, 0x2d, 0x4a, 0x55, 0x4c, 0x2d, 0x32, 0x30,
            0x32, 0x33, 0x00, 0x31, 0x30, 0x3a, 0x31, 0x39, 0x3a, 0x31, 0x38, 0x00, 0x31, 0x36,
            0x39, 0x00, 0x00, 0x00, 0x00, 0x00, 0xca, 0x06, 0x78, 0x78, 0x0a, 0xaa, 0x11, 0x0a,
            0x00, 0xba, 0x98, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0xca, 0x03, 0x60, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let want = FileHeader {
            data_size_index: DataSizeIndex::X250Bytes,
            total_length: 512,
            data_length: 283,
            datetime: DateTime::parse_from_rfc3339("2023-07-27T10:19:19.690Z").unwrap().to_utc(),
            sensor_available: SensorAvailable::NotPresent,
            motion: MotionConfig {
                direction: Direction::Clockwise,
                transducer: Transducer::Up,
                mode: Mode::Polar,
                step_size: StepSize::Fast,
            },
            start_gain: 6,
            sector_size: 360,
            train_angle: 360,
            range_code: RangeIndex::X1m,
            absorption: 1.7,
            config: Config {
                profile_grid: ProfileGrid::Off,
                zero: Zero::Up,
                data_bits: DataBits::X14Bits,
                logf: Logf::X20dB,
            },
            pulse_length: 100,
            sound_velocity: 1500.0,
            operating_frequency: 8,
            real_time_prf: 189.47,
            sensor_information: SensorInformation {
                pitch_valid: true,
                roll_valid: true,
                distance_valid: true,
            },
            pitch: 0.0,
            roll: 0.0,
            distance: 0.0,
        };

        let mut cursor = Cursor::new(sample_bytes);
        let got = FileHeader::read(&mut cursor).expect("It should not return an error");

        assert_eq!(want, got);
    }

    // #[test]
    // fn test_write() {
    //     let want: [u8; 108] = [
    //         0x02, 0x02, 0x00, 0x01, 0x1b, 0x32, 0x37, 0x2d,
    //         0x4a, 0x55, 0x4c, 0x2d, 0x32, 0x30, 0x32, 0x33,
    //         0x00, 0x31, 0x30, 0x3a, 0x31, 0x39, 0x3a, 0x31,
    //         0x38, 0x00, 0x31, 0x36, 0x39, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0xca, 0x06, 0x78, 0x78, 0x0a, 0xaa,
    //         0x11, 0x0a, 0x00, 0xba, 0x98, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0xca, 0x03,
    //         0x60, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00
    //     ];
    //
    //     let sample = FileHeader {
    //         data_size_index: DataSizeIndex::X250Bytes,
    //         total_length: 512,
    //         data_length: 283,
    //         datetime: DateTime::parse_from_rfc3339("2023-07-27T10:19:19.690Z").unwrap().to_utc(),
    //         sensor_available: SensorAvailable::NotPresent,
    //         motion: MotionConfig {
    //             direction: Direction::Clockwise,
    //             transducer: Transducer::Up,
    //             mode: Mode::Polar,
    //             step_size: StepSize::Fast
    //         },
    //         start_gain: 6,
    //         sector_size: 360,
    //         train_angle: 360,
    //         range_code: RangeIndex::X1m,
    //         absorption: 1.7,
    //         config: Config {
    //             profile_grid: ProfileGrid::Off,
    //             zero: Zero::Up,
    //             data_bits: DataBits::X14Bits,
    //             logf: Logf::X20dB
    //         },
    //         pulse_length: 100,
    //         sound_velocity: 1500.0,
    //         operating_frequency: 8,
    //         real_time_prf: 189.47,
    //         sensor_information: SensorInformation {
    //             pitch_valid: true,
    //             roll_valid: true,
    //             distance_valid: true
    //         },
    //         pitch: 0.0,
    //         roll: 0.0,
    //         distance: 0.0
    //     };
    //
    //     let mut cursor = Cursor::new(Vec::new());
    //     sample.write_be(&mut cursor).expect("It should not return an error");
    //
    //     let inner = cursor.into_inner();
    //     let got = inner.as_slice();
    //
    //     assert_eq!(want, got);
    // }
}
