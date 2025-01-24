use binrw::{BinResult, Error};
use num_traits::{FromPrimitive, ToPrimitive};
use std::any::type_name;

const ERR_MESSAGE_INVALID_VALUE: &str = "invalid value for";

const ERR_MESSAGE_CONVERT: &str = "failed to convert raw value for ";

pub(crate) fn read_u8<T>(raw: u8, pos: u64) -> BinResult<T>
where
    T: FromPrimitive,
{
    T::from_u8(raw).ok_or_else(|| Error::AssertFail {
        pos,
        message: format!("{} {}", ERR_MESSAGE_INVALID_VALUE, type_name::<T>().to_lowercase()),
    })
}

pub(crate) fn read_u8_bits<T>(raw: u8, mask: u8, shift: usize, pos: u64) -> BinResult<T>
where
    T: FromPrimitive,
{
    read_u8::<T>((raw & mask) >> shift, pos)
}

pub(crate) fn write_u8<T>(value: T, pos: u64) -> BinResult<u8>
where
    T: ToPrimitive,
{
    value.to_u8().ok_or_else(|| Error::AssertFail {
        pos,
        message: format!("{} {}", ERR_MESSAGE_CONVERT, type_name::<T>().to_lowercase()),
    })
}

pub(crate) fn write_u8_bits<T>(value: T, mask: u8, shift: usize, pos: u64) -> BinResult<u8>
where
    T: ToPrimitive,
{
    let raw = write_u8(value, pos)?;
    Ok((raw << shift) & mask)
}
