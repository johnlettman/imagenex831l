use binrw::meta::{EndianKind, ReadEndian, WriteEndian};

#[derive(Debug, derive_new::new)]
pub struct Data {
    pub echos: Option<Vec<f32>>,
}

impl ReadEndian for Data {
    const ENDIAN: EndianKind = EndianKind::None;
}

impl WriteEndian for Data {
    const ENDIAN: EndianKind = EndianKind::None;
}

// impl BinRead for Data {
//     type Args<'a> = (SonarReturnMagic, RangeIndex);
//
//     fn read_args<R: Read + Seek>(reader: &mut R, args: Self::Args<'_>) -> BinResult<Self>
//     where
//         Self: ReadEndian,
//     {
//         let (magic, range_index) = args;
//         let length = magic.data_length();
//
//         if length == 0 {
//             return Ok(Data::new(None));
//         }
//
//         let raw_args = VecArgs::builder().count(length).finalize();
//         let raw_echos: Vec<u8> = Vec::<u8>::read_args(reader, raw_args)?;
//
//
//
//     }
//
//
// }
