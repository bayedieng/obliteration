use std::mem::MaybeUninit;

macro_rules! read_le {
    ($ty:ty, $from:ident, $offset:ident, $size:literal) => {{
        let mut buf: [u8; $size] = uninit();
        let from = unsafe { $from.offset($offset as _) };

        unsafe { from.copy_to_nonoverlapping(buf.as_mut_ptr(), $size) };

        <$ty>::from_le_bytes(buf)
    }};
}

macro_rules! read_be {
    ($ty:ty, $from:ident, $offset:ident, $size:literal) => {{
        let mut buf: [u8; $size] = uninit();
        let from = unsafe { $from.offset($offset as _) };

        unsafe { from.copy_to_nonoverlapping(buf.as_mut_ptr(), $size) };

        <$ty>::from_be_bytes(buf)
    }};
}

macro_rules! write_le {
    ($to:ident, $offset:ident, $value:ident, $size:literal) => {{
        let bytes = $value.to_le_bytes();
        let to = unsafe { $to.offset($offset as _) };

        unsafe { to.copy_from_nonoverlapping(bytes.as_ptr(), $size) };
    }};
}

macro_rules! write_be {
    ($to:ident, $offset:ident, $value:ident, $size:literal) => {{
        let bytes = $value.to_be_bytes();
        let to = unsafe { $to.offset($offset as _) };

        unsafe { to.copy_from_nonoverlapping(bytes.as_ptr(), $size) };
    }};
}

/// Just a shortcut to `MaybeUninit::uninit().assume_init()`.
pub fn uninit<T>() -> T {
    unsafe { MaybeUninit::uninit().assume_init() }
}

pub fn read_u16_le(p: *const u8, i: usize) -> u16 {
    read_le!(u16, p, i, 2)
}

pub fn write_u16_le(p: *mut u8, i: usize, v: u16) {
    write_le!(p, i, v, 2)
}

pub fn read_u16_be(p: *const u8, i: usize) -> u16 {
    read_be!(u16, p, i, 2)
}

pub fn write_u16_be(p: *mut u8, i: usize, v: u16) {
    write_be!(p, i, v, 2)
}

pub fn read_u32_le(p: *const u8, i: usize) -> u32 {
    read_le!(u32, p, i, 4)
}

pub fn write_u32_le(p: *mut u8, i: usize, v: u32) {
    write_le!(p, i, v, 4)
}

pub fn read_u32_be(p: *const u8, i: usize) -> u32 {
    read_be!(u32, p, i, 4)
}

pub fn write_u32_be(p: *mut u8, i: usize, v: u32) {
    write_be!(p, i, v, 4)
}

pub fn read_u64_le(p: *const u8, i: usize) -> u64 {
    read_le!(u64, p, i, 8)
}

pub fn write_u64_le(p: *mut u8, i: usize, v: u64) {
    write_le!(p, i, v, 8)
}

pub fn read_u64_be(p: *const u8, i: usize) -> u64 {
    read_be!(u64, p, i, 8)
}

pub fn write_u64_be(p: *mut u8, i: usize, v: u64) {
    write_be!(p, i, v, 8)
}