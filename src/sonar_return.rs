use crate::types::SonarReturnHeader;
use binrw::{BinRead, BinWrite};

#[derive(Debug, BinRead, BinWrite, Clone)]
#[br(assert(termination_byte == 0xFC))]
#[bw(assert(* termination_byte == 0xFC))]
pub struct SonarReturn {
    header: SonarReturnHeader,
    #[br(
        count = header.data_length,
    )]
    data: Vec<u8>,
    termination_byte: u8,
}

impl SonarReturn {}
