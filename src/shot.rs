use crate::{types::FileHeader, SonarReturn};
use binrw::{BinRead, BinWrite};

#[derive(Debug, BinRead, BinWrite, Clone)]
#[brw(big, magic = b"31L")]
pub struct Shot {
    #[brw(pad_after = 117)]
    pub header: FileHeader,

    #[brw(pad_after = 1)]
    pub sonar_return: SonarReturn,
}
