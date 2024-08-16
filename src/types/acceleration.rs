use crate::types::primitive::i14f2;
use binrw::{BinRead, BinResult, BinWrite, Endian};
use std::cmp::Ordering;
use std::io::{Read, Seek, Write};

#[derive(Debug, Clone, derive_new::new)]
pub struct Acceleration {
    acceleration: f32,
    new_data: bool,
    error_alarm: bool,
}

impl Acceleration {
    const SCALE: f32 = 0.24414;
}

impl PartialEq for Acceleration {
    fn eq(&self, other: &Self) -> bool {
        self.acceleration.eq(&other.acceleration)
    }
}

impl PartialOrd for Acceleration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.acceleration.partial_cmp(&other.acceleration)
    }
}

impl BinRead for Acceleration {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let (raw, new_data, error_alarm) = i14f2::parse(reader, endian, args)?;
        let acceleration = raw as f32 * Acceleration::SCALE;
        Ok(Self::new(acceleration, new_data, error_alarm))
    }
}

impl BinWrite for Acceleration {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        let raw = (self.acceleration / Acceleration::SCALE) as i16;
        let values = (raw, self.new_data, self.error_alarm);
        i14f2::write(&values, writer, endian, args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use log::info;
    use test_log::test;

    #[test]
    fn test_eq() {
        let cases = vec![
            (Acceleration::new(6.0, true, false), Acceleration::new(6.0, true, false), true),
            (Acceleration::new(12.0, true, false), Acceleration::new(12.0, true, false), true),
            (Acceleration::new(32.0, true, false), Acceleration::new(60.0, true, false), false),
            (Acceleration::new(86.0, true, false), Acceleration::new(62.0, true, false), false),
        ];

        for (a, b, want) in cases {
            info!("Testing equality between {a:?} and {b:?}, expecting {want:?}");
            assert_eq!(a.eq(&b), want);
        }
    }

    #[test]
    fn test_ord() {
        let cases = vec![
            (
                Acceleration::new(6.0, true, false),
                Acceleration::new(6.0, true, false),
                Some(Ordering::Equal),
            ),
            (
                Acceleration::new(12.0, true, false),
                Acceleration::new(12.0, true, false),
                Some(Ordering::Equal),
            ),
            (
                Acceleration::new(32.0, true, false),
                Acceleration::new(60.0, true, false),
                Some(Ordering::Less),
            ),
            (
                Acceleration::new(86.0, true, false),
                Acceleration::new(62.0, true, false),
                Some(Ordering::Greater),
            ),
        ];

        for (a, b, want) in cases {
            info!("Comparing {a:?} and {b:?}, expecting {want:?}");
            assert_eq!(a.partial_cmp(&b), want);
        }
    }
}
