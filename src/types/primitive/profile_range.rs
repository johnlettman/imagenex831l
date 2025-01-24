use crate::types::{primitive::u14, RangeCode};
use binrw::{parser, writer, BinResult};

pub(crate) const SCALE: f32 = 0.0005;

#[parser(reader, endian)]
pub fn parse(range_index: RangeCode) -> BinResult<f32> {
    let value = u14::parse(reader, endian, ())?;
    let profile_range = (value as f32 * SCALE).floor() + range_index.filter_delay();
    Ok(profile_range)
}

#[writer(writer, endian)]
pub fn write(profile_range: &f32, range_index: &RangeCode) -> BinResult<()> {
    let value = ((*profile_range - range_index.filter_delay()) / SCALE).floor() as u16;
    u14::write(&value, writer, endian, ())?;
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::types::primitive::u14;
//     use crate::types::RangeIndex;
//     use binrw::{io::Cursor, BinRead, BinWrite, Endian};
//     use std::ops::Range;
//
//     use log::info;
//     use test_log::test;
//
//     const BINARY_ENDIAN: Endian = Endian::Little;
//     const BINARY_CASES: [(f32, [u8; 2], RangeIndex); 3] =
//         [
//             (0.0, [0x00, 0x00], RangeIndex::X25cm),
//             (16.383, [0b0111_1111, 0b0111_1111], RangeIndex::X2m),
//             (0.682625, [0b0010_1010, 0b0101_0101],RangeIndex::X12_5cm)
//         ];
//
//     #[test]
//     fn test_parse() {
//         for &(want, bytes, range_index) in BINARY_CASES.iter() {
//             info!("Parsing {bytes:?} with RangeIndex {range_index:?}, want {want:?}");
//             let mut cursor = Cursor::new(bytes);
//             let got = parse(&mut cursor, BINARY_ENDIAN, (range_index,))
//                 .expect("It should not return an error");
//             assert!((got - want).abs() < f32::EPSILON);
//         }
//     }
//
//     #[test]
//     fn test_write() {
//         for (profile_range, want, range_index) in BINARY_CASES.iter() {
//             info!("Writing {profile_range:?} with RangeIndex {range_index:?}, want {want:?}");
//             let mut cursor = Cursor::new(Vec::new());
//             write(&profile_range, &mut cursor, BINARY_ENDIAN, (range_index,))
//                 .expect("It should not return an error");
//             let inner = cursor.into_inner();
//             let got = inner.as_slice();
//             assert_eq!(want, got);
//         }
//     }
// }
