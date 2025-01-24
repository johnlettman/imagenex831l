use binrw::{parser, writer, BinResult, Error};
use std::str::from_utf8;

#[parser(reader)]
pub fn parse(length: usize) -> BinResult<(String, u64)> {
    let mut buffer = vec![0u8; length];
    let pos = reader.stream_position()?;
    reader.read_exact(&mut buffer)?;

    let string = from_utf8(&buffer)
        .map_err(|e| Error::Custom { pos, err: Box::new(e) })?
        .trim_end_matches('\0')
        .to_string();

    Ok((string, pos))
}

#[writer(writer)]
pub fn write(data: String, length: usize) -> BinResult<()> {
    let mut buffer = vec![0u8; length];
    let pos = writer.stream_position()?;
    let buffer_length = data.len().min(length);

    buffer[..buffer_length].copy_from_slice((&data[..buffer_length]).as_ref());
    writer.write_all(&buffer).map_err(|e| Error::Custom { pos, err: Box::new(e) })
}

#[cfg(test)]
mod tests {
    use super::*;

    use binrw::Endian;
    use log::info;
    use std::io::Cursor;
    use test_log::test;

    const BINARY_ENDIAN: Endian = Endian::NATIVE;
    const BINARY_CASES: [(&[u8], usize, &'static str); 3] = [
        (b"HELLO WORLD\0", 12, "HELLO WORLD"),
        (b"ASDF\0", 5, "ASDF"),
        (b"how do you do?\0", 15, "how do you do?"),
    ];

    #[test]
    fn test_parse() {
        for (bytes, length, want) in BINARY_CASES {
            info!("Parsing {bytes:?} with length {length}, expecting {want:?}");
            let mut cursor = Cursor::new(bytes);

            let (got, _) = parse(&mut cursor, BINARY_ENDIAN, (length,))
                .expect("It should not return an error");

            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_write() {
        for (want, length, string) in BINARY_CASES {
            info!("Writing {string:?} with length {length}, expecting {want:?}");
            let mut cursor = Cursor::new(Vec::new());

            write(string.to_string(), &mut cursor, BINARY_ENDIAN, (length,))
                .expect("It should not return an error");

            let got_inner = cursor.into_inner();
            let got = got_inner.as_slice();

            assert_eq!(want, got);
        }
    }
}
