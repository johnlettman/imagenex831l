use crate::{Result, Shot};
use binrw::BinRead;
#[cfg(feature = "pyo3")]
use pyo3::{exceptions::PyIOError, intern, prelude::*, types::PyString};
#[cfg(feature = "pyo3")]
use pyo3_file::PyFileLikeObject;
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};
#[cfg(not(target_family = "wasm"))]
use std::os::fd::{AsRawFd, FromRawFd};
use std::path::Path;
use std::{fs, slice};

#[cfg_attr(feature = "pyo3", pyclass)]
pub struct Reader {
    cursor: Cursor<Vec<u8>>,
    map: Option<memmap2::Mmap>,
}

impl Reader {
    pub fn new(inner: Vec<u8>) -> Self {
        Self { cursor: Cursor::new(inner), map: None }
    }

    #[inline]
    pub fn position(&self) -> u64 {
        self.cursor.position()
    }

    #[inline]
    pub fn is_mapped(&self) -> bool {
        self.map.is_some()
    }

    #[inline]
    pub fn map_ref(&self) -> Option<&memmap2::Mmap> {
        self.map.as_ref()
    }

    #[cfg(not(target_family = "wasm"))]
    #[inline]
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::from_file(fs::OpenOptions::new().read(true).open(path)?)
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn from_file(file: fs::File) -> Result<Self> {
        // Attempt to use memory mapping
        match unsafe { memmap2::Mmap::map(&file) } {
            Ok(map) => {
                #[cfg(unix)]
                map.advise(memmap2::Advice::Sequential).unwrap_or(());

                let cursor =
                    Cursor::new(unsafe { slice::from_raw_parts(map.as_ptr(), map.len()).to_vec() });

                Ok(Self { cursor, map: Some(map) })
            },
            Err(_) => {
                // Fallback
                let mut data = Vec::new();
                file.take(u64::MAX).read_to_end(&mut data)?;
                Ok(Self::new(data))
            },
        }
    }
}

impl Iterator for Reader {
    type Item = Shot;

    fn next(&mut self) -> Option<Self::Item> {
        match Shot::read(&mut self.cursor) {
            Ok(shot) => Some(shot),
            Err(_) => None,
        }
    }
}

impl Seek for Reader {
    #[inline]
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.cursor.seek(pos)
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl Reader {
    #[new]
    pub(crate) fn py_new(vec: Vec<u8>) -> Self {
        Self::new(vec)
    }

    pub(crate) fn __repr__(&self) -> String {
        format!(
            "<imagenex831l.Reader {}>",
            match self.map {
                Some(_) => "mapped",
                None => "unmapped",
            }
        )
    }

    pub(crate) fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    pub(crate) fn __next__(mut slf: PyRefMut<Self>) -> Option<Shot> {
        slf.next()
    }

    #[pyo3(name = "from_path")]
    #[staticmethod]
    pub(crate) fn py_from_path(path: String) -> PyResult<Self> {
        Self::from_path(path).map_err(|e| PyIOError::new_err(e.to_string()))
    }

    #[pyo3(name = "from_file")]
    #[staticmethod]
    pub(crate) fn py_from_file(file_like: PyObject) -> PyResult<Self> {
        Python::with_gil(|py| {
            // detect Path-like objects (e.g., strings)
            if let Ok(path) = file_like.downcast_bound::<PyString>(py) {
                return Reader::from_path(path.to_string_lossy().to_string())
                    .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()));
            }

            #[cfg(unix)]
            let has_fileno = file_like.call_method0(py, intern!(py, "fileno")).is_ok();
            match PyFileLikeObject::with_requirements(file_like, true, false, false, false) {
                Ok(mut f) => {
                    #[cfg(unix)]
                    if has_fileno {
                        let fd = f.as_raw_fd();
                        let file = unsafe { File::from_raw_fd(fd) };
                        Reader::from_file(file).map_err(|e| {
                            pyo3::exceptions::PyIOError::new_err(format!(
                                "Failed to create Reader: {e}"
                            ))
                        })
                    }

                    // If no valid file descriptor, fall back to reading data into memory
                    let mut data = Vec::new();
                    f.read_to_end(&mut data)?;
                    Ok(Reader::new(data))
                },
                Err(e) => Err(pyo3::exceptions::PyTypeError::new_err(format!(
                    "Invalid file-like object: {e}"
                ))),
            }
        })
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

        let mut shot_file = Reader::from_path(&path).expect("Failed to open shot file for reading");
        for shot in &mut shot_file {
            println!("{shot:#?}");
            break;
        }
    }
}
