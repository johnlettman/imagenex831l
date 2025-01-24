use crate::types::SonarReturnHeader;
use binrw::{BinRead, BinWrite};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[derive(Debug, BinRead, BinWrite, PartialEq, Clone)]
#[br(assert(termination_byte == 0xFC))]
#[bw(assert(* termination_byte == 0xFC))]
#[cfg_attr(feature = "pyo3", pyclass(eq))]
pub struct SonarReturn {
    #[cfg(not(feature = "pyo3"))]
    header: SonarReturnHeader,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    header: SonarReturnHeader,

    #[cfg(not(feature = "pyo3"))]
    #[br(count = header.data_length,)]
    data: Vec<u8>,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    #[br(count = header.data_length,)]
    data: Vec<u8>,

    #[cfg(not(feature = "pyo3"))]
    termination_byte: u8,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    termination_byte: u8,
}
