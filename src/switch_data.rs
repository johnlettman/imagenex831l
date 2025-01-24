use crate::types::{Command, RangeCode};
use binrw::{BinRead, BinWrite};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[derive(Debug, BinRead, BinWrite, Eq, PartialEq, derive_new::new)]
#[brw(magic = b"\xFE\x44")]
#[cfg_attr(feature = "pyo3", pyclass(eq))]
pub struct SwitchData {
    #[cfg(not(feature = "pyo3"))]
    #[brw(pad_before = 1)]
    #[brw(pad_after = 1)]
    range_index: RangeCode,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    #[brw(pad_before = 1)]
    #[brw(pad_after = 1)]
    range_index: RangeCode,

    #[cfg(not(feature = "pyo3"))]
    #[brw(pad_after = 1)]
    command: Command,

    #[cfg(feature = "pyo3")]
    #[pyo3(get, set)]
    #[brw(pad_after = 1)]
    command: Command,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl SwitchData {
    #[new]
    fn py_new(range_index: RangeCode, command: Command) -> Self {
        Self::new(range_index, command)
    }

    pub fn __repr__(&self) -> String {
        format!("SwitchData(range_index = {:?}, command = {:?})", self.range_index, self.command)
    }
}
