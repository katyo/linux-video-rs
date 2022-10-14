use crate::{Error, Result};

use core::str;
use std::io;

#[inline(always)]
pub fn invalid_input(msg: &'static str) -> Error {
    Error::new(io::ErrorKind::InvalidInput, msg)
}

#[inline(always)]
pub fn invalid_data(msg: &'static str) -> Error {
    Error::new(io::ErrorKind::InvalidData, msg)
}

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
