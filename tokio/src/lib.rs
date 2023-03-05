#![forbid(future_incompatible)]
#![deny(bad_style/*, missing_docs*/)]
#![doc = include_str!("../README.md")]

use std::{
    fs::File,
    io,
    os::unix::io::{AsRawFd, RawFd},
    path::{Path, PathBuf},
};

pub use linux_video_core as types;
use linux_video_core::private::*;
use types::*;

use tokio::{io::unix::AsyncFd, task::spawn_blocking};

async fn asyncify<F, T>(f: F) -> Result<T>
where
    F: FnOnce() -> Result<T> + Send + 'static,
    T: Send + 'static,
{
    match spawn_blocking(f).await {
        Ok(res) => res,
        Err(_) => Err(Error::new(io::ErrorKind::Other, "background task failed")),
    }
}

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
    pub async fn list() -> Result<Devices> {
        Devices::new().await
    }

    /// Open video device
    pub async fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_owned();
        let file = asyncify(move || open(path, true)).await?;
        //let file = File::from_file(file)?;

        Ok(Device { file })
    }

    /// Get capabilities
    pub async fn capabilities(&self) -> Result<Capability> {
        let fd = self.as_raw_fd();
        asyncify(move || Internal::<Capability>::query(fd).map(Internal::into_inner)).await
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
    pub async fn control(&self, id: impl Into<u32>) -> Result<Control> {
        let fd = self.as_raw_fd();
        let id = id.into();
        let ctrl = asyncify(move || Internal::<QueryExtCtrl>::query_fallback(fd, id)).await?;

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
    pub async fn get_control<T: GetValue>(&self, value: &mut T) -> Result<()> {
        //let fd = self.as_raw_fd();
        //asyncify(move || value.get(fd)).await
        value.get(self.as_raw_fd())
    }

    /// Set control value
    pub async fn set_control<T: SetValue>(&self, value: &T) -> Result<()> {
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
    pub async fn format(&self, type_: BufferType) -> Result<Format> {
        let fd = self.as_raw_fd();
        asyncify(move || {
            let mut fmt = Format::from(type_);
            Internal::from(&mut fmt).get(fd)?;
            Ok(fmt)
        })
        .await
    }

    /// Get current format
    pub async fn get_format(&self, fmt: &mut Format) -> Result<()> {
        let fmt_ = self.format(fmt.type_()).await?;
        fmt.clone_from(&fmt_);
        Ok(())
    }

    /// Set current format
    pub async fn set_format(&self, fmt: &mut Format) -> Result<()> {
        let fd = self.as_raw_fd();
        let mut fmt2 = *fmt;
        *fmt = asyncify(move || -> Result<Format> {
            Internal::from(&mut fmt2).set(fd)?;
            Ok(fmt2)
        })
        .await?;
        Ok(())
    }

    /// Try format without set it
    pub async fn try_format(&self, fmt: &mut Format) -> Result<()> {
        let fd = self.as_raw_fd();
        let mut fmt2 = *fmt;
        *fmt = asyncify(move || -> Result<Format> {
            Internal::from(&mut fmt2).try_(fd)?;
            Ok(fmt2)
        })
        .await?;
        Ok(())
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
    reader: tokio::fs::ReadDir,
}

impl Devices {
    async fn new() -> Result<Self> {
        tokio::fs::read_dir("/dev")
            .await
            .map(|reader| Devices { reader })
    }

    /// Get path of the next device
    pub async fn fetch_next(&mut self) -> Result<Option<PathBuf>> {
        use std::os::unix::fs::FileTypeExt;

        while let Some(entry) = self.reader.next_entry().await? {
            if let Some(file_name) = entry.file_name().to_str() {
                if check_dev_name(file_name).is_some() {
                    let file_type = entry.file_type().await?;
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
    pub async fn fetch_next(&mut self) -> Result<Option<Control>> {
        if self.last_id == u32::MAX {
            return Ok(None);
        }

        let fd = self.device.as_raw_fd();
        let id = self.last_id;

        if let Some(ctrl) =
            asyncify(move || Internal::<QueryExtCtrl>::query_next_fallback(fd, id)).await?
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
    pub async fn fetch_next(&mut self) -> Result<Option<MenuItem>> {
        let fd = self.device.as_raw_fd();
        let type_ = self.ctrl_type;
        let id = self.ctrl_id;
        for index in &mut self.index_iter {
            if let Some(item) =
                asyncify(move || Internal::<MenuItem>::query(fd, type_, id, index)).await?
            {
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
    pub async fn fetch_next(&mut self) -> Result<Option<FmtDesc>> {
        if self.index == u32::MAX {
            return Ok(None);
        }

        let fd = self.device.as_raw_fd();
        let index = self.index;
        let type_ = self.type_;

        if let Some(desc) = asyncify(move || Internal::<FmtDesc>::query(fd, index, type_)).await? {
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
    pub async fn fetch_next(&mut self) -> Result<Option<FrmSizeEnum>> {
        if self.index == u32::MAX {
            return Ok(None);
        }

        let fd = self.device.as_raw_fd();
        let index = self.index;
        let pixfmt = self.pixel_format;

        if let Some(size) =
            asyncify(move || Internal::<FrmSizeEnum>::query(fd, index, pixfmt)).await?
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
    pub async fn fetch_next(&mut self) -> Result<Option<FrmIvalEnum>> {
        if self.index == u32::MAX {
            return Ok(None);
        }

        let fd = self.device.as_raw_fd();
        let index = self.index;
        let pixfmt = self.pixel_format;
        let width = self.width;
        let height = self.height;

        if let Some(ival) =
            asyncify(move || Internal::<FrmIvalEnum>::query(fd, index, pixfmt, width, height))
                .await?
        {
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

struct FdWrapper {
    fd: RawFd,
}

impl AsRawFd for FdWrapper {
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

impl<Dir: Direction, Met: Method> Stream<Dir, Met> {
    fn new(file: File, type_: ContentType, count: usize) -> Result<Self> {
        let queue = Internal::<QueueData<Dir, Met>>::new(file.as_raw_fd(), type_, count as _)?;

        Ok(Self { file, queue })
    }

    /// Get next frame to write or read
    pub async fn next(&self) -> Result<BufferData<'_, Dir, Met>> {
        let fd = self.file.as_raw_fd();

        loop {
            match self.queue.next(fd) {
                Ok(buffer) => break Ok(buffer),
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    let async_fd = AsyncFd::new(FdWrapper { fd })?;

                    let _ = core::future::poll_fn(|cx| {
                        if Dir::IN {
                            async_fd.poll_read_ready(cx)
                        } else {
                            async_fd.poll_write_ready(cx)
                        }
                    })
                    .await?;
                }
                Err(error) => break Err(error),
            }
        }
    }
}
