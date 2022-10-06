use crate::{Error, Result};

use std::{io, mem::size_of_val, str};

#[inline(always)]
pub fn invalid_input(msg: &'static str) -> Error {
    Error::new(io::ErrorKind::InvalidInput, msg)
}

#[inline(always)]
pub fn invalid_data(msg: &'static str) -> Error {
    Error::new(io::ErrorKind::InvalidData, msg)
}

#[inline(always)]
pub fn check_len_str<T: ?Sized>(slice: &str, val: &T) -> Result<()> {
    if slice.as_bytes().len() /* \0 */ < size_of_val(val) {
        Ok(())
    } else {
        Err(invalid_input("String too long"))
    }
}

/*
#[inline(always)]
pub fn set_str<const N: usize>(dst: &mut [u8; N], src: &str) -> Result<()> {
    check_len_str(src, dst)?;

    let src = src.as_bytes();
    dst[..src.len()].copy_from_slice(src);
    dst[src.len()] = 0;

    Ok(())
}
*/

#[inline(always)]
fn is_null(src: &u8) -> bool {
    *src == 0
}

#[inline(always)]
fn get_ascii(src: &[u8]) -> &[u8] {
    src.splitn(2, is_null).next().unwrap()
}

#[inline(always)]
pub fn check_str(src: &[u8]) -> Result<()> {
    str::from_utf8(get_ascii(src))
        .map_err(|_| invalid_data("Invalid UTF-8"))
        .map(|_| ())
}

#[inline(always)]
pub fn get_str(src: &[u8]) -> Result<&str> {
    str::from_utf8(get_ascii(src)).map_err(|_| invalid_data("Invalid UTF-8"))
}

#[inline(always)]
pub fn get_str_unchecked(src: &[u8]) -> &str {
    unsafe { str::from_utf8_unchecked(get_ascii(src)) }
}
