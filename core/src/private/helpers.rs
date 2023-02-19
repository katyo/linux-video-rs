use crate::{Result, ContentType};
use std::{
    fs::{File, OpenOptions},
    os::unix::fs::{FileTypeExt, OpenOptionsExt},
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

    if !path.metadata()?.file_type().is_char_device() {
        return Err(crate::utils::invalid_input("No character device"));
    }

    pub const O_NONBLOCK: i32 = 2048;

    OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(if nonblock { O_NONBLOCK } else { 0 })
        .open(path)
}

/// Check video device name prefix
pub fn check_dev_name(name: impl AsRef<str>) -> Option<ContentType> {
    let name = name.as_ref();
    if name.starts_with("video") {
        Some(ContentType::Video)
    } else if name.starts_with("vbi") {
        Some(ContentType::Vbi)
    } else if name.starts_with("radio") || name.starts_with("swradio") {
        Some(ContentType::Sdr)
    } else {
        None
    }
}
