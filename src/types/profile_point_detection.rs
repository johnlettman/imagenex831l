use num_derive::{FromPrimitive, ToPrimitive};

/// New in Ethernet Specification v1.01, revision `03` from March 22, 2023.
#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum ProfilePointDetection {
    CenterOfPulse = 0,
    StartOfPulse = 1,
}
