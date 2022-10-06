pub use v4l2_core::interface::*;
use v4l2_core::*;

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
    pub fn control_get(&self, control: &Control) -> Result<Value> {
        let mut value = Internal::<Value>::from(&control.ctrl);

        value.get(self.file.as_raw_fd())?;

        Ok(value.into_inner())
    }

    /// Set control value
    pub fn control_set(&self, value: &Value) -> Result<()> {
        Internal::from(value).set(self.file.as_raw_fd())
    }

    /// Get control values
    pub fn controls_get<'i>(
        &self,
        controls: impl IntoIterator<Item = &'i Control>,
    ) -> Result<Values> {
        let mut values: Internal<Values> = controls
            .into_iter()
            .map(|ctrl| ctrl.ctrl.as_ref())
            .collect();

        values.get(self.file.as_raw_fd())?;

        Ok(values.into_inner())
    }

    /// Set control values
    pub fn controls_set(&self, controls: &mut Values) -> Result<()> {
        Internal::from(controls).set(self.file.as_raw_fd())
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
