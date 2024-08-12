use crate::Error;
use crate::Shot;
use binrw::{BinRead, BinWrite};
use memmap2::{Mmap, MmapMut};
use std::io::{Cursor, Error as IOError, ErrorKind, Seek, SeekFrom};
use std::path::Path;
use std::{fs, slice};

pub enum FileMode {
    Read,
    Write,
}

pub struct File {
    map_ro: Option<Mmap>,
    map_rw: Option<MmapMut>,
    cursor: Cursor<&'static [u8]>,
    mode: FileMode,
}

impl File {
    const ERR_MESSAGE_RO: &'static str = "File not opened for writing";

    pub fn open<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let file = fs::File::open(path)?;
        let map_ro = unsafe { Mmap::map(&file)? };
        let cursor = Cursor::new(unsafe { slice::from_raw_parts(map_ro.as_ptr(), map_ro.len()) });

        Ok(Self { map_ro: Some(map_ro), map_rw: None, cursor, mode: FileMode::Read })
    }

    pub fn create<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(path)?;
        let map_rw = unsafe { MmapMut::map_mut(&file)? };
        let cursor = Cursor::new(unsafe { slice::from_raw_parts(map_rw.as_ptr(), map_rw.len()) });

        Ok(Self { map_ro: None, map_rw: Some(map_rw), cursor, mode: FileMode::Write })
    }

    pub fn write_shot(&mut self, shot: &Shot) -> crate::Result<()> {
        match &mut self.map_rw {
            Some(map_rw) => {
                let mut cursor = Cursor::new(unsafe {
                    slice::from_raw_parts_mut(map_rw.as_mut_ptr(), map_rw.len())
                });

                cursor.seek(SeekFrom::End(0))?;
                shot.write(&mut cursor).map_err(|e| Error::BinaryEncoding(e))
            },
            None => Err(IOError::new(ErrorKind::Other, Self::ERR_MESSAGE_RO).into()),
        }
    }

    pub fn close(mut self) -> crate::Result<()> {
        if let Some(map_rw) = self.map_rw.take() {
            map_rw.flush()?;
        }

        Ok(())
    }
}

impl Drop for File {
    fn drop(&mut self) {
        if let Some(map_rw) = &self.map_rw {
            map_rw.flush().expect("Failed to flush changes to the disk");
        }
    }
}

impl Iterator for File {
    type Item = Shot;

    fn next(&mut self) -> Option<Self::Item> {
        match self.mode {
            FileMode::Read => match Shot::read(&mut self.cursor) {
                Ok(item) => Some(item),
                Err(_) => None,
            },
            FileMode::Write => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use project_root::get_project_root;

    #[test]
    fn test_shot_file_reading_and_writing() {
        let mut path = get_project_root().unwrap();
        path.push("sample");
        path.push("27JUL2023-101914.31l");

        let mut shot_file = File::open(&path).expect("Failed to open shot file for reading");
        for shot in &mut shot_file {
            println!("{shot:#?}");
            break;
        }
    }
}
