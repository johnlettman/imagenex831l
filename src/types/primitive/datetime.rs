use crate::types::primitive::string;
use binrw::{parser, writer, BinResult, Endian, Error};
use chrono::{DateTime, NaiveDate, NaiveTime, TimeDelta, Timelike, Utc};
use std::io::Seek;
use std::str::from_utf8;

const DATE_FORMAT: &'static str = "%d-%b-%Y";
const TIME_FORMAT: &'static str = "%H:%M:%S";

const DATE_BYTES_LENGTH: usize = 12;
const TIME_BYTES_LENGTH: usize = 9;
const HUNDREDTHS_BYTES_LENGTH: usize = 4;
const TOTAL_LENGTH: usize = DATE_BYTES_LENGTH + TIME_BYTES_LENGTH + HUNDREDTHS_BYTES_LENGTH;

const ENDIAN: Endian = Endian::NATIVE;

const SCALE_NANOSECONDS_TO_HUNDREDTHS: u32 = 10_000_000;

#[parser(reader)]
pub fn parse() -> BinResult<DateTime<Utc>> {
    let (date_string, date_pos) = string::parse(reader, ENDIAN, (DATE_BYTES_LENGTH,))?;
    let (time_string, time_pos) = string::parse(reader, ENDIAN, (TIME_BYTES_LENGTH,))?;
    let (hundredths_string, hundredths_pos) =
        string::parse(reader, ENDIAN, (HUNDREDTHS_BYTES_LENGTH,))?;

    let date = NaiveDate::parse_from_str(date_string.as_str(), DATE_FORMAT)
        .map_err(|e| Error::Custom { pos: date_pos, err: Box::new(e) })?;

    let time = NaiveTime::parse_from_str(time_string.as_str(), TIME_FORMAT)
        .map_err(|e| Error::Custom { pos: time_pos, err: Box::new(e) })?;

    let hundredths: u32 = hundredths_string
        .trim_start_matches('.')
        .parse()
        .map_err(|e| Error::Custom { pos: hundredths_pos, err: Box::new(e) })?;

    let time = time + TimeDelta::nanoseconds((hundredths * SCALE_NANOSECONDS_TO_HUNDREDTHS) as i64);
    let datetime = DateTime::from_naive_utc_and_offset(date.and_time(time), Utc);
    Ok(datetime)
}

#[writer(writer)]
pub fn write(datetime: &DateTime<Utc>) -> BinResult<()> {
    let date_string = datetime.format(DATE_FORMAT).to_string().to_uppercase();
    let time_string = datetime.format(TIME_FORMAT).to_string();

    let nanoseconds = datetime.timestamp_subsec_nanos();
    let hundredths = nanoseconds / SCALE_NANOSECONDS_TO_HUNDREDTHS;
    let hundredths_string = format!(".{hundredths:02}");

    string::write(date_string, writer, ENDIAN, (DATE_BYTES_LENGTH,))?;
    string::write(time_string, writer, ENDIAN, (TIME_BYTES_LENGTH,))?;
    string::write(hundredths_string, writer, ENDIAN, (HUNDREDTHS_BYTES_LENGTH,))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    use binrw::io::Cursor;
    use binrw::Endian;
    use chrono::{TimeDelta, TimeZone, Utc};
    use log::info;
    use test_log::test;

    const BINARY_ENDIAN: Endian = Endian::NATIVE;

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

            let got = parse(&mut cursor, BINARY_ENDIAN, ()).expect("It should not return an error");

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

            write(&datetime, &mut cursor, BINARY_ENDIAN, ())
                .expect("It should not return an error");

            let inner = cursor.into_inner();
            let got = inner.as_slice();

            let want_str = from_utf8(want).unwrap();
            let got_str = from_utf8(got).unwrap();

            assert_eq!(want_str, got_str);
        }
    }
}
