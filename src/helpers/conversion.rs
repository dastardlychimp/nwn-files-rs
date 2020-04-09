use std::convert::TryInto;

#[inline]
pub fn u32_from_bytes(b: &[u8]) -> u32 {
    u32::from_le_bytes(b.try_into().unwrap())
}

#[inline]
pub fn u16_from_bytes(b: &[u8]) -> u16 {
    u16::from_le_bytes(b.try_into().unwrap())
}
