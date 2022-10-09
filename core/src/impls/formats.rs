use crate::{calls, types::*, utils, Internal, Result};
use core::mem::MaybeUninit;
use std::os::unix::io::RawFd;

impl Internal<FmtDesc> {
    pub fn query(fd: RawFd, index: u32, type_: BufferType) -> Result<Option<Self>> {
        let mut q = MaybeUninit::<FmtDesc>::zeroed();

        unsafe_call!({
            {
                let q = q.assume_init_mut();

                q.index = index;
                q.type_ = type_;
            }
            calls::enum_fmt(fd, q.as_mut_ptr()).map(|_| q.assume_init())
        })
        .and_then(|q| {
            utils::check_str(&q.description)?;
            Ok(Some(q.into()))
        })
        .or_else(|error| {
            if error.kind() == std::io::ErrorKind::InvalidInput {
                Ok(None)
            } else {
                Err(error)
            }
        })
    }
}

trivial_impls! {
    FmtDesc {
        /// Format description
        getstr description: &str,
    }
}

impl core::fmt::Display for FmtDesc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        '#'.fmt(f)?;
        self.index.fmt(f)?;
        ' '.fmt(f)?;
        self.type_.fmt(f)?;
        ' '.fmt(f)?;
        self.flags.fmt(f)?;
        f.write_str(" '")?;
        self.description().fmt(f)?;
        f.write_str("' ")?;
        self.pixel_format.fmt(f)?;
        f.write_str(" @")?;
        self.mbus_code.fmt(f)
    }
}
