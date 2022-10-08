use crate::{CtrlType, QueryExtCtrl, Result};
use std::os::unix::io::RawFd;

pub trait PlainData {
    /// Control types
    const TYPES: &'static [CtrlType];
}

pub trait RefValue<T> {
    /// Get reference to value
    fn try_ref<'a>(data: &'a T, ctrl: &QueryExtCtrl) -> Option<&'a Self>;
}

pub trait MutValue<T> {
    /// Get mutable reference to value
    fn try_mut<'a>(data: &'a mut T, ctrl: &QueryExtCtrl) -> Option<&'a mut Self>;
}

pub trait GetValue {
    /// Get value from device
    fn get(&mut self, fd: RawFd) -> Result<()>;
}

pub trait SetValue {
    /// Set value to device
    fn set(&self, fd: RawFd) -> Result<()>;
}
