use binrw::{BinRead, BinWrite};
use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt::{Display, Formatter};

/// Size index of the sonar data field.
/// The IMAGENEX documentation refers to this as `nToReadIndex`.
#[derive(Debug, BinRead, BinWrite, Eq, PartialEq, Copy, Clone, ToPrimitive, FromPrimitive)]
#[repr(u8)]
#[brw(repr = u8)]
pub enum DataSizeIndex {
    X250Bytes = 2,
}

impl DataSizeIndex {
    pub const fn bytes(&self) -> usize {
        match *self {
            Self::X250Bytes => 250,
        }
    }
}

impl Default for DataSizeIndex {
    fn default() -> Self {
        Self::X250Bytes
    }
}

impl Display for DataSizeIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::X250Bytes => "250 bytes",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use log::info;
    use test_log::test;

    #[test]
    fn test_bytes() {
        let cases = vec![(DataSizeIndex::X250Bytes, 250usize)];

        for (data_size_index, want) in cases {
            info!("Getting bytes for {data_size_index:?}, expecting {want}");
            let got = data_size_index.bytes();
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_default() {
        let want = DataSizeIndex::X250Bytes;
        let got = DataSizeIndex::default();
        assert_eq!(got, want, "it should default to {want:?}");
    }

    #[test]
    fn test_display() {
        let cases = vec![(DataSizeIndex::X250Bytes, "250 bytes")];

        for (data_size_index, want) in cases {
            info!("Displaying {data_size_index:?}, expecting {want:?}");
            let got = format!("{data_size_index}");
            assert_eq!(want, got);
        }
    }
}
