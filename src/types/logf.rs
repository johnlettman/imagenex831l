use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Copy, Clone, ToPrimitive, FromPrimitive)]
pub enum Logf {
    X10dB = 0,
    X20dB = 1,
    X30dB = 2,
    X40dB = 3,
}

impl Logf {
    pub const fn decibels(&self) -> usize {
        match *self {
            Logf::X10dB => 10,
            Logf::X20dB => 20,
            Logf::X30dB => 30,
            Logf::X40dB => 40,
        }
    }
}

impl Display for Logf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::X10dB => "10 dB",
                Self::X20dB => "20 dB",
                Self::X30dB => "30 dB",
                Self::X40dB => "40 dB",
            }
        )
    }
}

impl Ord for Logf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_u8().cmp(&other.to_u8())
    }
}

impl PartialOrd for Logf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_u8().partial_cmp(&other.to_u8())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use log::info;
    use test_log::test;

    #[test]
    fn test_decibels() {
        let cases = vec![
            (Logf::X10dB, 10usize),
            (Logf::X20dB, 20usize),
            (Logf::X30dB, 30usize),
            (Logf::X40dB, 40usize),
        ];

        for (logf, want) in cases {
            info!("Getting decibels for {logf:?}, expecting {want}");
            let got = logf.decibels();
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_display() {
        let cases = vec![
            (Logf::X10dB, "10 dB"),
            (Logf::X20dB, "20 dB"),
            (Logf::X30dB, "30 dB"),
            (Logf::X40dB, "40 dB"),
        ];

        for (logf, want) in cases {
            info!("Displaying {logf:?}, expecting {want:?}");
            let got = format!("{logf}");
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_ord() {
        let cases = vec![
            (Logf::X10dB, Logf::X10dB, Ordering::Equal),
            (Logf::X10dB, Logf::X30dB, Ordering::Less),
            (Logf::X40dB, Logf::X20dB, Ordering::Greater),
        ];

        for (a, b, want) in cases {
            info!("Comparing {a:?} and {b:?}, expecting {want:?}");

            let got = a.partial_cmp(&b).expect("Should not be None");
            assert_eq!(want, got);

            let got = a.cmp(&b);
            assert_eq!(want, got);
        }
    }
}
