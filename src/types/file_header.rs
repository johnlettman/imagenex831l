use crate::types::{
    primitive::{
        absorption, datetime, pulse_length, real_time_prf, sector_size, sound_velocity, start_gain,
        train_angle,
    },
    Config, DataPoints, MotionConfig, RangeCode, SensorAvailable, SensorInformation,
};
use binrw::{BinRead, BinWrite};
use chrono::{DateTime, Utc};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

// If you're reading this monster created from the lack of
// #[cfg_attr()] for the pyo3 macros
// I am so very sorry.

#[derive(Debug, BinRead, BinWrite, PartialEq, Clone)]
#[cfg_attr(
    target_family = "wasm",
    derive(tsify::Tsify, serde::Serialize, serde::Deserialize),
    tsify(into_wasm_abi, from_wasm_abi)
)]
#[cfg_attr(
    all(feature = "serde", not(target_family = "wasm")),
    derive(serde::Serialize, serde::Deserialize)
)]
#[brw(big)]
#[br(
    assert(total_length == Self::VALID_TOTAL_LENGTH),
    assert(data_length == Self::VALID_DATA_LENGTH)
)]
#[bw(
    assert(* total_length == Self::VALID_TOTAL_LENGTH),
    assert(* data_length == Self::VALID_DATA_LENGTH)
)]
#[cfg_attr(feature = "pyo3", pyclass(eq))]
pub struct FileHeader {
    #[cfg(not(feature = "pyo3"))]
    data_size_index: DataPoints,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    data_size_index: DataPoints,

    #[cfg(not(feature = "pyo3"))]
    total_length: u16,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    total_length: u16,

    #[cfg(not(feature = "pyo3"))]
    data_length: u16,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    data_length: u16,

    #[cfg(not(feature = "pyo3"))]
    #[br(parse_with = datetime::parse)]
    #[bw(write_with = datetime::write)]
    #[brw(pad_after = 1)]
    datetime: DateTime<Utc>,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    #[br(parse_with = datetime::parse)]
    #[bw(write_with = datetime::write)]
    #[brw(pad_after = 1)]
    datetime: DateTime<Utc>,

    #[cfg(not(feature = "pyo3"))]
    #[brw(pad_after = 2)]
    sensor_available: SensorAvailable,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    #[brw(pad_after = 2)]
    sensor_available: SensorAvailable,

    #[cfg(not(feature = "pyo3"))]
    motion: MotionConfig,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    motion: MotionConfig,

    #[cfg(not(feature = "pyo3"))]
    #[br(parse_with = start_gain::parse)]
    #[bw(write_with = start_gain::write)]
    start_gain: u8,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    #[br(parse_with = start_gain::parse)]
    #[bw(write_with = start_gain::write)]
    start_gain: u8,

    #[cfg(not(feature = "pyo3"))]
    #[br(parse_with = sector_size::parse)]
    #[bw(write_with = sector_size::write)]
    sector_size: u16,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    #[br(parse_with = sector_size::parse)]
    #[bw(write_with = sector_size::write)]
    sector_size: u16,

    #[cfg(not(feature = "pyo3"))]
    #[br(parse_with = train_angle::parse)]
    #[bw(write_with = train_angle::write)]
    train_angle: u16,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    #[br(parse_with = train_angle::parse)]
    #[bw(write_with = train_angle::write)]
    train_angle: u16,

    #[cfg(not(feature = "pyo3"))]
    range_code: RangeCode,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    range_code: RangeCode,

    #[cfg(not(feature = "pyo3"))]
    #[br(parse_with = absorption::parse)]
    #[bw(write_with = absorption::write)]
    absorption: f32,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    #[br(parse_with = absorption::parse)]
    #[bw(write_with = absorption::write)]
    absorption: f32,

    #[cfg(not(feature = "pyo3"))]
    config: Config,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    config: Config,

    #[cfg(not(feature = "pyo3"))]
    #[br(parse_with = pulse_length::parse)]
    #[bw(write_with = pulse_length::write)]
    #[brw(pad_after = 1)]
    pulse_length: u16,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    #[br(parse_with = pulse_length::parse)]
    #[bw(write_with = pulse_length::write)]
    #[brw(pad_after = 1)]
    pulse_length: u16,

    #[cfg(not(feature = "pyo3"))]
    #[br(parse_with = sound_velocity::parse)]
    #[bw(write_with = sound_velocity::write)]
    #[brw(pad_after = 31)]
    sound_velocity: f32,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    #[br(parse_with = sound_velocity::parse)]
    #[bw(write_with = sound_velocity::write)]
    #[brw(pad_after = 31)]
    sound_velocity: f32,

    #[cfg(not(feature = "pyo3"))]
    operating_frequency: u16,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    operating_frequency: u16,

    #[cfg(not(feature = "pyo3"))]
    #[br(parse_with = real_time_prf::parse)]
    #[bw(write_with = real_time_prf::write)]
    #[brw(pad_after = 15)]
    real_time_prf: f32,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    #[br(parse_with = real_time_prf::parse)]
    #[bw(write_with = real_time_prf::write)]
    #[brw(pad_after = 15)]
    real_time_prf: f32,

    #[cfg(not(feature = "pyo3"))]
    sensor_information: SensorInformation,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    sensor_information: SensorInformation,

    #[cfg(not(feature = "pyo3"))]
    pitch: f32,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    pitch: f32,

    #[cfg(not(feature = "pyo3"))]
    roll: f32,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    roll: f32,

    #[cfg(not(feature = "pyo3"))]
    distance: f32,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    distance: f32,
}

impl FileHeader {
    pub(crate) const VALID_TOTAL_LENGTH: u16 = 512;
    pub(crate) const VALID_DATA_LENGTH: u16 = 283;
}
