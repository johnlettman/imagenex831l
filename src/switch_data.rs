use crate::types::{Command, RangeIndex};
use binrw::{BinRead, BinWrite};

#[derive(Debug, BinRead, BinWrite)]
#[brw(magic = b"\xFE\x44")]
pub struct SwitchData {
    #[brw(pad_before = 1)]
    #[brw(pad_after = 1)]
    range_index: RangeIndex,

    #[brw(pad_after = 1)]
    command: Command,
}
