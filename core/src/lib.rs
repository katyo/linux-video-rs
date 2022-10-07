#![doc = include_str!("../README.md")]

#[cfg(not(target_os = "linux"))]
compile_error!("This crate support Linux only");

#[macro_use]
mod macros;

mod calls;
mod consts;
mod ctrlid;
mod enums;
mod fourcc;
mod impls;
mod stdid;
mod structs;
mod utils;

pub mod types {
    pub use crate::{consts::*, ctrlid::*, enums::*, fourcc::*, impls::*, stdid::*, structs::*};
}

use std::{
    fs::{File, OpenOptions},
    os::unix::fs::OpenOptionsExt,
    path::Path,
};

pub use std::io::{Error, Result};

#[repr(transparent)]
pub struct Internal<T>(pub T);

impl<T> From<T> for Internal<T> {
    fn from(this: T) -> Self {
        Self(this)
    }
}

impl<T> Internal<T> {
    pub fn into_inner(self) -> T {
        self.0
    }

    pub fn map<R>(self, mapper: impl FnOnce(T) -> R) -> Internal<R> {
        mapper(self.into_inner()).into()
    }
}

impl<T> core::ops::Deref for Internal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> core::ops::DerefMut for Internal<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for Internal<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Internal<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

/// Open device by path or name
pub fn open(path: impl AsRef<Path>, nonblock: bool) -> Result<File> {
    let path = path.as_ref();

    #[allow(unused)]
    let mut full_path = None;

    let path = if path.is_absolute() {
        path
    } else {
        full_path = Some(Path::new("dev").join(path));
        full_path.as_ref().unwrap()
    };

    pub const O_NONBLOCK: i32 = 2048;

    OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(if nonblock { O_NONBLOCK } else { 0 })
        .open(path)
}
