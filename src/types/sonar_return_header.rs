use crate::types::{
    primitive::{profile_range, u14},
    Acceleration, Angle, HeadPosition, RangeCode, SonarReturnMagic, SonarReturnStatus, SonarType,
};
use binrw::{BinRead, BinWrite};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[derive(Debug, BinRead, BinWrite, PartialEq, Clone, derive_new::new)]
#[cfg_attr(
    target_family = "wasm",
    derive(tsify::Tsify, serde::Serialize, serde::Deserialize),
    tsify(into_wasm_abi, from_wasm_abi)
)]
#[cfg_attr(
    all(feature = "serde", not(target_family = "wasm")),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(feature = "pyo3", pyclass)]
pub struct SonarReturnHeader {
    #[cfg(not(feature = "pyo3"))]
    pub magic: SonarReturnMagic,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    pub magic: SonarReturnMagic,

    #[cfg(not(feature = "pyo3"))]
    pub sonar_type: SonarType,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    pub sonar_type: SonarType,

    #[cfg(not(feature = "pyo3"))]
    pub status: SonarReturnStatus,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    pub status: SonarReturnStatus,

    #[cfg(not(feature = "pyo3"))]
    pub head_position: HeadPosition,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    pub head_position: HeadPosition,

    #[cfg(not(feature = "pyo3"))]
    pub range_code: RangeCode,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    pub range_code: RangeCode,

    #[cfg(not(feature = "pyo3"))]
    #[br(parse_with = profile_range::parse)]
    #[bw(write_with = profile_range::write)]
    #[br(args(range_code))]
    #[bw(args(range_code))]
    pub profile_range: f32,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    #[br(parse_with = profile_range::parse)]
    #[bw(write_with = profile_range::write)]
    #[br(args(range_code))]
    #[bw(args(range_code))]
    pub profile_range: f32,

    #[cfg(not(feature = "pyo3"))]
    #[br(parse_with = u14::parse)]
    #[bw(write_with = u14::write)]
    #[brw(pad_after = 4)]
    pub data_length: u16,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    #[br(parse_with = u14::parse)]
    #[bw(write_with = u14::write)]
    #[brw(pad_after = 4)]
    pub data_length: u16,

    #[cfg(not(feature = "pyo3"))]
    pub roll_angle: Angle,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    pub roll_angle: Angle,

    #[cfg(not(feature = "pyo3"))]
    pub pitch_angle: Angle,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    pub pitch_angle: Angle,

    #[cfg(not(feature = "pyo3"))]
    pub roll_acceleration: Acceleration,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    pub roll_acceleration: Acceleration,

    #[cfg(not(feature = "pyo3"))]
    #[brw(pad_after = 8)]
    pub pitch_acceleration: Acceleration,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    #[brw(pad_after = 8)]
    pub pitch_acceleration: Acceleration,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl SonarReturnHeader {
    #[new]
    pub(crate) fn py_new(
        magic: SonarReturnMagic,
        sonar_type: SonarType,
        status: SonarReturnStatus,
        head_position: HeadPosition,
        range_code: RangeCode,
        profile_range: f32,
        data_length: u16,
        roll_angle: Angle,
        pitch_angle: Angle,
        roll_acceleration: Acceleration,
        pitch_acceleration: Acceleration,
    ) -> Self {
        Self::new(
            magic,
            sonar_type,
            status,
            head_position,
            range_code,
            profile_range,
            data_length,
            roll_angle,
            pitch_angle,
            roll_acceleration,
            pitch_acceleration,
        )
    }
}
