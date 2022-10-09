pub use v4l2_core as types;
use v4l2_core::private::*;

use types::*;

use std::{
    fs::File,
    os::unix::io::{AsRawFd, RawFd},
    path::Path,
};

/// Video device
pub struct Device {
    file: File,
}

impl AsRawFd for Device {
    fn as_raw_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}

impl Device {
    /// Open video device
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let file = open(path, true)?;

        Ok(Device { file })
    }

    /// Get capabilities
    pub fn capabilities(&self) -> Result<Capability> {
        Internal::<Capability>::query(self.as_raw_fd()).map(Internal::into_inner)
    }

    /// Get controls
    pub fn controls(&self) -> Controls<'_> {
        Controls {
            device: self,
            last_id: 0,
        }
    }

    /// Get control by identifier
    pub fn control(&self, id: impl AsRef<u32>) -> Result<Control> {
        let ctrl = Internal::<QueryExtCtrl>::query_fallback(self.as_raw_fd(), *id.as_ref())?;

        Ok(Control { ctrl })
    }

    /// Get control menu items
    pub fn control_items<'i, 'c>(&'i self, control: &'c Control) -> Option<MenuItems<'i>> {
        if control.is_menu() {
            Some(MenuItems {
                device: self,
                ctrl_type: control.type_(),
                ctrl_id: control.id(),
                index_iter: control.min() as _..=control.max() as _,
            })
        } else {
            None
        }
    }

    /// Get control value
    pub fn get_control<T: GetValue>(&self, value: &mut T) -> Result<()> {
        value.get(self.as_raw_fd())
    }

    /// Set control value
    pub fn set_control<T: SetValue>(&self, value: &T) -> Result<()> {
        value.set(self.as_raw_fd())
    }

    /// Get supported formats
    pub fn formats(&self, type_: BufferType) -> FmtDescs {
        FmtDescs {
            device: self,
            type_,
            index: 0,
        }
    }

    /// Get current format
    pub fn format(&self, type_: BufferType) -> Result<Format> {
        let mut fmt = Format::from(type_);
        self.get_format(&mut fmt)?;
        Ok(fmt)
    }

    /// Get current format
    pub fn get_format(&self, fmt: &mut Format) -> Result<()> {
        Internal::from(fmt).get(self.as_raw_fd())
    }

    /// Set current format
    pub fn set_format(&self, fmt: &Format) -> Result<()> {
        Internal::from(fmt).set(self.as_raw_fd())
    }
}

pub struct Controls<'i> {
    device: &'i Device,
    last_id: u32,
}

impl<'i> Iterator for Controls<'i> {
    type Item = Result<Control>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.last_id == u32::MAX {
            return None;
        }

        match Internal::<QueryExtCtrl>::query_next_fallback(self.device.as_raw_fd(), self.last_id) {
            Ok(Some(ctrl)) => {
                self.last_id = ctrl.id();
                Some(Ok(Control { ctrl }))
            }
            Ok(None) => {
                self.last_id = u32::MAX;
                None
            }
            Err(error) => Some(Err(error)),
        }
    }
}

impl<'i> core::iter::FusedIterator for Controls<'i> {}

pub struct Control {
    ctrl: Internal<QueryExtCtrl>,
}

impl core::ops::Deref for Control {
    type Target = QueryExtCtrl;

    fn deref(&self) -> &Self::Target {
        &self.ctrl
    }
}

impl AsRef<QueryExtCtrl> for Control {
    fn as_ref(&self) -> &QueryExtCtrl {
        &self.ctrl
    }
}

impl core::fmt::Display for Control {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.ctrl.fmt(f)
    }
}

pub struct MenuItems<'i> {
    device: &'i Device,
    ctrl_type: CtrlType,
    ctrl_id: u32,
    index_iter: core::ops::RangeInclusive<u32>,
}

impl<'i> Iterator for MenuItems<'i> {
    type Item = Result<MenuItem>;

    fn next(&mut self) -> Option<Self::Item> {
        for index in &mut self.index_iter {
            if let Some(item) = Internal::<MenuItem>::query(
                self.device.as_raw_fd(),
                self.ctrl_type,
                self.ctrl_id,
                index,
            )
            .transpose()
            {
                return Some(item.map(Internal::into_inner));
            }
        }
        None
    }
}

impl<'i> core::iter::FusedIterator for MenuItems<'i> {}

pub struct FmtDescs<'i> {
    device: &'i Device,
    type_: BufferType,
    index: u32,
}

impl<'i> Iterator for FmtDescs<'i> {
    type Item = Result<FmtDesc>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == u32::MAX {
            return None;
        }

        match Internal::<FmtDesc>::query(self.device.as_raw_fd(), self.index, self.type_) {
            Ok(Some(desc)) => {
                self.index += 1;
                Some(Ok(desc.into_inner()))
            }
            Ok(None) => {
                self.index = u32::MAX;
                None
            }
            Err(error) => Some(Err(error)),
        }
    }
}

impl<'i> core::iter::FusedIterator for FmtDescs<'i> {}
