//! Utilities for the **Date Time** primitive, a string-based timestamp.
//!
//! ## Wire format
//! This primitive is encoded as two null-terminated C strings:
//! - Date: 12 bytes, `DD-MMM-YYYY` format,
//! - Time: 9 bytes, `HH:MM:SS` format, and
//! - Sub-seconds: 4 bytes,
//!     - *PipeSonarL* up to version 1012: `.hh` (_hundredths of seconds_), or
//!     - *PipeSonarL* version 1013 and above: `mmm` (_milliseconds_)
use crate::types::primitive::string;
use crate::ENDIAN;
use binrw::{parser, writer, BinResult, Error};
use chrono::{DateTime, NaiveDate, NaiveTime, TimeDelta, Utc};

pub(crate) const DATE_FORMAT: &str = "%d-%b-%Y";
pub(crate) const TIME_FORMAT: &str = "%H:%M:%S";

pub(crate) const DATE_LENGTH: usize = 12;
pub(crate) const TIME_LENGTH: usize = 9;
pub(crate) const SUB_LENGTH: usize = 4;

const SCALE_NANOSECONDS_TO_HUNDREDTHS: u32 = 10_000_000;

/// The format of the sub-seconds component of the **Date Time** primitive.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SubFormat {
    /// Hundredths of Seconds (`.hh`) for *PipeSonarL* up to version 1012.
    Hundredths,

    /// Milliseconds (`mmm`) for *PipeSonarL* version 1013 and above.
    Milliseconds,
}

impl Default for SubFormat {
    fn default() -> Self {
        Self::Hundredths
    }
}

/// Parse a **Date Time** from a sequence of bytes.
#[parser(reader)]
pub fn parse() -> BinResult<DateTime<Utc>> {
    let (date_string, date_pos) = string::parse(reader, ENDIAN, (DATE_LENGTH,))?;
    let (time_string, time_pos) = string::parse(reader, ENDIAN, (TIME_LENGTH,))?;
    let (sub_string, sub_pos) = string::parse(reader, ENDIAN, (SUB_LENGTH,))?;

    let date = NaiveDate::parse_from_str(date_string.as_str(), DATE_FORMAT)
        .map_err(|e| Error::Custom { pos: date_pos, err: Box::new(e) })?;

    let mut time = NaiveTime::parse_from_str(time_string.as_str(), TIME_FORMAT)
        .map_err(|e| Error::Custom { pos: time_pos, err: Box::new(e) })?;

    if sub_string.starts_with('.') {
        // The presence of the starting `.` indicates this is the hundredths format.
        let hundredths: u32 = sub_string
            .trim_start_matches('.')
            .parse()
            .map_err(|e| Error::Custom { pos: sub_pos, err: Box::new(e) })?;

        time += TimeDelta::nanoseconds((hundredths * SCALE_NANOSECONDS_TO_HUNDREDTHS) as i64);
    } else {
        // Otherwise, this is the milliseconds format.
        let milliseconds =
            sub_string.parse().map_err(|e| Error::Custom { pos: sub_pos, err: Box::new(e) })?;
        time += TimeDelta::milliseconds(milliseconds);
    }

    let datetime = DateTime::from_naive_utc_and_offset(date.and_time(time), Utc);
    Ok(datetime)
}

/// Write a **Date Time** to a sequence of bytes.
#[writer(writer)]
pub fn write(datetime: &DateTime<Utc>, sub_format: SubFormat) -> BinResult<()> {
    let date_string = datetime.format(DATE_FORMAT).to_string().to_uppercase();
    let time_string = datetime.format(TIME_FORMAT).to_string();
    let sub_string = match sub_format {
        SubFormat::Hundredths => {
            let hundredths = datetime.timestamp_subsec_nanos() / SCALE_NANOSECONDS_TO_HUNDREDTHS;
            format!(".{hundredths:02}")
        },
        SubFormat::Milliseconds => {
            let milliseconds = datetime.timestamp_millis();
            format!("{milliseconds:03}")
        },
    };

    string::write(date_string, writer, ENDIAN, (DATE_LENGTH,))?;
    string::write(time_string, writer, ENDIAN, (TIME_LENGTH,))?;
    string::write(sub_string, writer, ENDIAN, (SUB_LENGTH,))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::from_utf8;

    use binrw::io::Cursor;
    use binrw::Endian;
    use chrono::{TimeDelta, TimeZone, Utc};
    use log::info;
    use test_log::test;

    #[test]
    fn sub_format_default() {
        assert_eq!(SubFormat::Hundredths, SubFormat::default());
    }

    #[test]
    fn test_parse() {
        let cases = [
            (
                b"01-JAN-2023\012:34:56\0.23\0",
                Utc.with_ymd_and_hms(2023, 1, 1, 12, 34, 56).unwrap()
                    + TimeDelta::nanoseconds((23 * SCALE_NANOSECONDS_TO_HUNDREDTHS) as i64),
            ),
            (
                b"01-JAN-2023\012:34:56\0.99\0",
                Utc.with_ymd_and_hms(2023, 1, 1, 12, 34, 56).unwrap()
                    + TimeDelta::nanoseconds(990_000_000), // Near 1 second
            ),
            (
                b"01-JAN-2023\012:34:57\0.00\0",
                Utc.with_ymd_and_hms(2023, 1, 1, 12, 34, 56).unwrap()
                    + TimeDelta::nanoseconds(1_000_000_000), // Exactly 1 second
            ),
        ];

        for (bytes, want) in cases {
            info!("Parsing {bytes:?}, expecting {want:?}");
            let mut cursor = Cursor::new(bytes);
            let got = parse(&mut cursor, ENDIAN, ()).expect("Should succeed");
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_write() {
        let cases = [
            (
                b"01-JAN-2023\012:34:56\0.23\0",
                Utc.with_ymd_and_hms(2023, 1, 1, 12, 34, 56).unwrap()
                    + TimeDelta::nanoseconds((23 * SCALE_NANOSECONDS_TO_HUNDREDTHS) as i64),
            ),
            (
                b"01-JAN-2023\012:34:56\0.99\0",
                Utc.with_ymd_and_hms(2023, 1, 1, 12, 34, 56).unwrap()
                    + TimeDelta::nanoseconds(999_000_000), // Near 1 second
            ),
            (
                b"01-JAN-2023\012:34:57\0.00\0",
                Utc.with_ymd_and_hms(2023, 1, 1, 12, 34, 56).unwrap()
                    + TimeDelta::nanoseconds(1_000_000_000), // Exactly 1 second
            ),
        ];

        for (want, datetime) in cases {
            info!("Writing {datetime:?}, expecting {want:?}");
            let mut cursor = Cursor::new(Vec::new());

            write(&datetime, &mut cursor, ENDIAN, (SubFormat::Hundredths,))
                .expect("It should not return an error");

            let inner = cursor.into_inner();
            let got = inner.as_slice();

            let want_str = from_utf8(want).unwrap();
            let got_str = from_utf8(got).unwrap();

            assert_eq!(want_str, got_str);
        }
    }
}
