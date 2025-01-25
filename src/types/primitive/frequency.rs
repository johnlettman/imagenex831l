//! Utilities for the **Frequency** primitive, a representation of the sonar frequency.
//!
//! ## Wire format
//! - [SonarType.Scanning]: 2.15 MHz to 2.35 MHz in 5 kHz increments, and
//! - [SonarType.FixedPosition]: 900 kHz to 1.10 MHz in 5 kHz increments.
use crate::types::SonarType;
use binrw::{parser, writer, BinRead, BinResult, BinWrite, Error};

pub(crate) const MIN_SCANNING: f32 = 2.15;
pub(crate) const MAX_SCANNING: f32 = 2.35;

pub(crate) const MIN_FIXED_POSITION: f32 = 0.9;
pub(crate) const MAX_FIXED_POSITION: f32 = 1.1;

const ERR_MESSAGE_RANGE: &str = "frequency exceeds range";

/// Obtain the center **Frequency** offset for the specified [SonarType].
fn offset_for(sonar_type: SonarType) -> f32 {
    match sonar_type {
        SonarType::Scanning => 2250.0,
        SonarType::FixedPosition => 1000.0,
    }
}

/// Validate the **Frequency** for the specified [SonarType].
pub fn valid_for(sonar_type: SonarType, frequency: f32) -> bool {
    match sonar_type {
        SonarType::Scanning => (MIN_SCANNING..=MAX_SCANNING).contains(&frequency),
        SonarType::FixedPosition => (MIN_FIXED_POSITION..=MAX_FIXED_POSITION).contains(&frequency),
    }
}

/// Parse the **Frequency** for the specified [SonarType] from a byte.
#[parser(reader)]
pub fn parse(sonar_type: SonarType) -> BinResult<f32> {
    let raw = u8::read(reader)?;
    let frequency = ((raw as f32 - offset_for(sonar_type)) / 5.0) + 100.0;

    if !valid_for(sonar_type, frequency) {
        let pos = reader.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_MESSAGE_RANGE.to_string() });
    }

    Ok(frequency)
}

/// Write the **Frequency** for the specified [SonarType] to a byte.
#[writer(writer)]
pub fn write(frequency: &f32, sonar_type: SonarType) -> BinResult<()> {
    if !valid_for(sonar_type, *frequency) {
        let pos = writer.stream_position()?;
        return Err(Error::AssertFail { pos, message: ERR_MESSAGE_RANGE.to_string() });
    }

    let raw = ((*frequency - 100.0) * 5.0 + offset_for(sonar_type)).ceil() as u8;
    raw.write(writer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use log::info;
    use test_log::test;

    #[test]
    fn test_offset_for() {
        let cases = vec![(SonarType::Scanning, 2250.0), (SonarType::FixedPosition, 1000.0)];

        for (sonar_type, want) in cases {
            info!("Getting offset for {sonar_type:?}, expecting {want}");
            let got = offset_for(sonar_type);
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_valid_for() {
        let cases = vec![
            (SonarType::Scanning, MAX_SCANNING + 1.2, false),
            (SonarType::Scanning, MIN_SCANNING - 0.3, false),
            (SonarType::FixedPosition, MAX_FIXED_POSITION + 3.4, false),
            (SonarType::FixedPosition, MIN_FIXED_POSITION - 0.9, false),
            (SonarType::Scanning, MAX_SCANNING - 0.001, true),
            (SonarType::Scanning, MIN_SCANNING + 0.003, true),
            (SonarType::FixedPosition, MAX_FIXED_POSITION - 0.05, true),
            (SonarType::FixedPosition, MIN_FIXED_POSITION + 0.09, true),
            (SonarType::Scanning, MAX_SCANNING, true),
            (SonarType::Scanning, MIN_SCANNING, true),
            (SonarType::FixedPosition, MAX_FIXED_POSITION, true),
            (SonarType::FixedPosition, MIN_FIXED_POSITION, true),
        ];

        for (sonar_type, frequency, want) in cases {
            info!("Checking validity of {frequency} for {sonar_type:?}, expecting {want}");
            let got = valid_for(sonar_type, frequency);
            assert_eq!(want, got);
        }
    }
}
