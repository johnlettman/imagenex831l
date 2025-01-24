use crate::{types::FileHeader, SonarReturn};
use binrw::{BinRead, BinWrite};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[derive(Debug, BinRead, BinWrite, PartialEq, Clone, derive_new::new)]
#[cfg_attr(feature = "pyo3", pyclass(eq))]
#[brw(big, magic = b"31L")]
pub struct Shot {
    #[cfg(not(feature = "pyo3"))]
    #[brw(pad_after = 117)]
    pub header: FileHeader,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    #[brw(pad_after = 117)]
    pub header: FileHeader,

    #[cfg(not(feature = "pyo3"))]
    #[brw(pad_after = 1)]
    pub sonar_return: SonarReturn,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    #[brw(pad_after = 1)]
    pub sonar_return: SonarReturn,
}
