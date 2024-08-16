use binrw::meta::{EndianKind, ReadEndian, WriteEndian};
use binrw::{BinRead, BinResult, BinWrite, Endian, Error};
use std::io::{Error as IOError, ErrorKind::InvalidData, Read, Seek, Write};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(
    target_family = "wasm",
    derive(tsify::Tsify, serde::Serialize, serde::Deserialize),
    tsify(into_wasm_abi, from_wasm_abi),
    serde(rename_all = "UPPERCASE")
)]
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

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        _: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<()> {
        self.write(writer)
    }
}
