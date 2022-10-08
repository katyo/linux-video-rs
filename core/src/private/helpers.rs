use crate::Result;
use std::{
    fs::{File, OpenOptions},
    os::unix::fs::OpenOptionsExt,
    path::Path,
};

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
