use binrw::meta::{EndianKind, ReadEndian, WriteEndian};
use binrw::{BinRead, BinResult, BinWrite, Endian, Error};
use std::fmt::{Display, Formatter};
use std::io::{Error as IOError, ErrorKind::InvalidData, Read, Seek, Write};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(
    target_family = "wasm",
    derive(tsify::Tsify, serde::Serialize, serde::Deserialize),
    tsify(into_wasm_abi, from_wasm_abi)
)]
#[cfg_attr(
    all(feature = "serde", not(target_family = "wasm")),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(feature = "pyo3", pyclass(eq))]
pub enum SonarReturnMagic {
    IMX,
    IPX,
}

impl SonarReturnMagic {
    pub fn data_length(&self) -> usize {
        match *self {
            Self::IMX => 250,
            Self::IPX => 0,
        }
    }
}

impl Display for SonarReturnMagic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::IMX => "IMX",
                Self::IPX => "IPX",
            }
        )
    }
}

impl ReadEndian for SonarReturnMagic {
    const ENDIAN: EndianKind = EndianKind::None;
}

impl WriteEndian for SonarReturnMagic {
    const ENDIAN: EndianKind = EndianKind::None;
}

impl BinRead for SonarReturnMagic {
    type Args<'a> = ();

    fn read<R: Read + Seek>(reader: &mut R) -> BinResult<Self>
    where
        Self: ReadEndian,
    {
        let mut raw = [0u8; 3];
        let pos = reader.stream_position()?;

        reader.read_exact(&mut raw)?;

        match &raw {
            b"IMX" => Ok(Self::IMX),
            b"IPX" => Ok(Self::IPX),
            _ => Err(Error::Custom {
                pos,
                err: Box::new(IOError::new(InvalidData, "Invalid return data magic")),
            }),
        }
    }

    #[inline]
    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<Self> {
        Self::read(reader)
    }
}

impl BinWrite for SonarReturnMagic {
    type Args<'a> = ();

    fn write<W: Write + Seek>(&self, writer: &mut W) -> BinResult<()>
    where
        Self: WriteEndian,
    {
        let raw = match *self {
            Self::IMX => b"IMX",
            Self::IPX => b"IPX",
        };

        writer.write(raw)?;
        Ok(())
    }

    #[inline]
    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        _: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<()> {
        self.write(writer)
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl SonarReturnMagic {
    pub(crate) fn __str__(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    use log::info;
    use test_log::test;

    #[test]
    fn test_data_length() {
        let cases = vec![(SonarReturnMagic::IMX, 250), (SonarReturnMagic::IPX, 0)];

        for (magic, want) in cases {
            info!("Getting data length for {magic:?}, want {want:?}");
            let got = magic.data_length();
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_display() {
        let cases = vec![(SonarReturnMagic::IMX, "IMX"), (SonarReturnMagic::IPX, "IPX")];

        for (magic, want) in cases {
            info!("Displaying {magic:?}, want {want:?}");
            let got = format!("{magic}");
            assert_eq!(want, got);
        }
    }

    const BINARY_ENDIAN: Endian = Endian::NATIVE;
    const BINARY_CASES: [(SonarReturnMagic, &[u8; 3]); 2] =
        [(SonarReturnMagic::IMX, b"IMX"), (SonarReturnMagic::IPX, b"IPX")];

    #[test]
    fn test_parse() {
        for (want, bytes) in BINARY_CASES {
            info!("Parsing {bytes:?}, want {want:?}");
            let mut cursor = Cursor::new(bytes);
            let got = SonarReturnMagic::read_options(&mut cursor, BINARY_ENDIAN, ())
                .expect("It should not return an error");
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_parse_invalid() {
        let cases = vec![b"ASD", b"QWE", b"TFD"];

        for bytes in cases {
            info!("Parsing {bytes:?}, want error");
            let mut cursor = Cursor::new(bytes);
            let error = SonarReturnMagic::read_options(&mut cursor, BINARY_ENDIAN, ()).unwrap_err();
            assert!(matches!(error, Error::Custom { .. }));
        }
    }

    #[test]
    fn test_write() {
        for (magic, want) in BINARY_CASES {
            info!("Writing {magic:?}, want {want:?}");
            let mut cursor = Cursor::new(Vec::new());
            magic
                .write_options(&mut cursor, BINARY_ENDIAN, ())
                .expect("It should not return an error");
            let inner = cursor.into_inner();
            let got = inner.as_slice();
            assert_eq!(want, got);
        }
    }
}
