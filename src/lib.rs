#![forbid(future_incompatible)]
#![deny(bad_style/*, missing_docs*/)]
#![doc = include_str!("../README.md")]

pub use linux_video_core as types;
use linux_video_core::private::*;
use types::*;

use std::{
    fs::File,
    os::unix::io::{AsRawFd, RawFd},
    path::{Path, PathBuf},
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
    /// List video devices
    pub fn list() -> Result<Devices> {
        Devices::new()
    }

    /// Open video device
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let file = open(path, false)?;

        Ok(Device { file })
    }

    /// Get capabilities
    pub fn capabilities(&self) -> Result<Capability> {
        Internal::<Capability>::query(self.as_raw_fd()).map(Internal::into_inner)
    }

    /// Get controls
    pub fn controls(&self, class: Option<CtrlClass>) -> Controls<'_> {
        let last_id = class.map(|c| c as _).unwrap_or_default();

        Controls {
            device: self,
            class,
            last_id,
        }
    }

    /// Get control by identifier
    pub fn control(&self, id: impl Into<u32>) -> Result<Control> {
        let ctrl = Internal::<QueryExtCtrl>::query_fallback(self.as_raw_fd(), id.into())?;

        Ok(Control { ctrl })
    }

    /// Get control menu items
    pub fn control_items(&self, control: &Control) -> Option<MenuItems<'_>> {
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
    pub fn set_format(&self, fmt: &mut Format) -> Result<()> {
        Internal::from(fmt).set(self.as_raw_fd())
    }

    /// Try format without set it
    pub fn try_format(&self, fmt: &mut Format) -> Result<()> {
        Internal::from(fmt).try_(self.as_raw_fd())
    }

    /// Get supported frame sizes
    pub fn sizes(&self, pixel_format: FourCc) -> FrmSizes {
        FrmSizes {
            device: self,
            pixel_format,
            index: 0,
        }
    }

    /// Get supported frame intervals
    pub fn intervals(&self, pixel_format: FourCc, width: u32, height: u32) -> FrmIvals {
        FrmIvals {
            device: self,
            pixel_format,
            width,
            height,
            index: 0,
        }
    }

    /// Create stream to input/output data
    pub fn stream<Dir: Direction, Met: Method>(
        &self,
        type_: ContentType,
        count: usize,
    ) -> Result<Stream<Dir, Met>> {
        Stream::new(self.file.try_clone()?, type_, count)
    }
}

/// The interface to get available devices
pub struct Devices {
    reader: std::fs::ReadDir,
}

impl Devices {
    fn new() -> Result<Self> {
        std::fs::read_dir("/dev").map(|reader| Devices { reader })
    }

    /// Get path of the next device
    pub fn fetch_next(&mut self) -> Result<Option<PathBuf>> {
        use std::os::unix::fs::FileTypeExt;

        for entry in self.reader.by_ref() {
            let entry = entry?;
            if let Some(file_name) = entry.file_name().to_str() {
                if check_dev_name(file_name).is_some() {
                    let file_type = entry.file_type()?;
                    if file_type.is_char_device() {
                        return Ok(Some(entry.path()));
                    }
                }
            }
        }

        Ok(None)
    }
}

/// The interface to get device controls
pub struct Controls<'i> {
    device: &'i Device,
    class: Option<CtrlClass>,
    last_id: u32,
}

impl<'i> Controls<'i> {
    /// Get next control
    pub fn fetch_next(&mut self) -> Result<Option<Control>> {
        if self.last_id == u32::MAX {
            return Ok(None);
        }

        if let Some(ctrl) =
            Internal::<QueryExtCtrl>::query_next_fallback(self.device.as_raw_fd(), self.last_id)?
        {
            if self
                .class
                .map(|class| class.fast_match(ctrl.id()))
                .unwrap_or(true)
            {
                self.last_id = ctrl.id();
                Ok(Some(Control { ctrl }))
            } else {
                self.last_id = u32::MAX;
                Ok(None)
            }
        } else {
            self.last_id = u32::MAX;
            Ok(None)
        }
    }
}

/// The control access interface
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

/// The interface to get menu items
pub struct MenuItems<'i> {
    device: &'i Device,
    ctrl_type: CtrlType,
    ctrl_id: u32,
    index_iter: core::ops::RangeInclusive<u32>,
}

impl<'i> MenuItems<'i> {
    /// Get next menu control item
    pub fn fetch_next(&mut self) -> Result<Option<MenuItem>> {
        for index in &mut self.index_iter {
            if let Some(item) = Internal::<MenuItem>::query(
                self.device.as_raw_fd(),
                self.ctrl_type,
                self.ctrl_id,
                index,
            )? {
                return Ok(Some(item.into_inner()));
            }
        }
        Ok(None)
    }
}

/// The interface to get format descriptions
pub struct FmtDescs<'i> {
    device: &'i Device,
    type_: BufferType,
    index: u32,
}

impl<'i> FmtDescs<'i> {
    /// Fetch next format description
    pub fn fetch_next(&mut self) -> Result<Option<FmtDesc>> {
        if self.index == u32::MAX {
            return Ok(None);
        }

        if let Some(desc) =
            Internal::<FmtDesc>::query(self.device.as_raw_fd(), self.index, self.type_)?
        {
            self.index += 1;
            Ok(Some(desc.into_inner()))
        } else {
            self.index = u32::MAX;
            Ok(None)
        }
    }
}

/// The interface to get drame sizes
pub struct FrmSizes<'i> {
    device: &'i Device,
    pixel_format: FourCc,
    index: u32,
}

impl<'i> FrmSizes<'i> {
    /// Get next frame size value
    pub fn fetch_next(&mut self) -> Result<Option<FrmSizeEnum>> {
        if self.index == u32::MAX {
            return Ok(None);
        }

        if let Some(size) =
            Internal::<FrmSizeEnum>::query(self.device.as_raw_fd(), self.index, self.pixel_format)?
        {
            self.index += 1;
            Ok(Some(size.into_inner()))
        } else {
            self.index = u32::MAX;
            Ok(None)
        }
    }
}

/// The interface to get frame intervals
pub struct FrmIvals<'i> {
    device: &'i Device,
    pixel_format: FourCc,
    width: u32,
    height: u32,
    index: u32,
}

impl<'i> FrmIvals<'i> {
    /// Get next frame interval value
    pub fn fetch_next(&mut self) -> Result<Option<FrmIvalEnum>> {
        if self.index == u32::MAX {
            return Ok(None);
        }

        if let Some(ival) = Internal::<FrmIvalEnum>::query(
            self.device.as_raw_fd(),
            self.index,
            self.pixel_format,
            self.width,
            self.height,
        )? {
            self.index += 1;
            Ok(Some(ival.into_inner()))
        } else {
            self.index = u32::MAX;
            Ok(None)
        }
    }
}

/// Data I/O queue
pub struct Stream<Dir, Met: Method> {
    file: File,
    queue: Internal<QueueData<Dir, Met>>,
}

impl<Dir, Met: Method> Drop for Stream<Dir, Met> {
    fn drop(&mut self) {
        let _ = self.queue.del(self.file.as_raw_fd());
    }
}

impl<Dir: Direction, Met: Method> Stream<Dir, Met> {
    fn new(file: File, type_: ContentType, count: usize) -> Result<Self> {
        let queue = Internal::<QueueData<Dir, Met>>::new(file.as_raw_fd(), type_, count as _)?;

        Ok(Self { file, queue })
    }

    /// Get next frame to write or read
    pub fn next(&self) -> Result<BufferRef<Dir, Met>> {
        self.queue.next(self.file.as_raw_fd())
    }
}

macro_rules! iter_impls {
    ($($type:ident $(<$($type_params:lifetime),*>)* => $item_type:ident,)*) => {
        $(
            impl $(<$($type_params),*>)* Iterator for $type $(<$($type_params),*>)* {
                type Item = Result<$item_type>;

                fn next(&mut self) -> Option<Self::Item> {
                    self.fetch_next().transpose()
                }
            }

            impl $(<$($type_params),*>)* core::iter::FusedIterator for $type $(<$($type_params),*>)* {}
        )*
    };
}

iter_impls! {
    Devices => PathBuf,
    Controls<'i> => Control,
    MenuItems<'i> => MenuItem,
    FmtDescs<'i> => FmtDesc,
    FrmSizes<'i> => FrmSizeEnum,
    FrmIvals<'i> => FrmIvalEnum,
}
