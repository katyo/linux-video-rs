use crate::{calls, types::*, Internal, Result};
use std::os::unix::io::RawFd;

impl Internal<&mut StreamParm> {
    pub fn get(&mut self, fd: RawFd) -> Result<()> {
        unsafe_call!(calls::g_parm(fd, *self.as_mut() as *mut _)).map(|_| ())
    }
}

impl Internal<&mut StreamParm> {
    pub fn set(&self, fd: RawFd) -> Result<()> {
        unsafe_call!(calls::s_parm(fd, *self.as_ref() as *const _ as *mut _)).map(|_| ())
    }
}

unsafe impl Send for StreamParm {}

impl From<BufferType> for StreamParm {
    fn from(type_: BufferType) -> Self {
        Self {
            type_,
            parm: if type_.is_capture() {
                StreamParmUnion {
                    capture: CaptureParm::default(),
                }
            } else {
                StreamParmUnion {
                    output: OutputParm::default(),
                }
            },
        }
    }
}

impl StreamParm {
    /// Create from value
    pub fn new<T: IsStreamParm>(type_: BufferType, data: T) -> Option<Self> {
        if T::OUTPUT == type_.is_output() {
            let mut this = Self::from(type_);
            *this.try_mut().unwrap() = data;
            Some(this)
        } else {
            None
        }
    }

    /// Get reference to value
    pub fn try_ref<T: IsStreamParm>(&self) -> Option<&T> {
        if T::OUTPUT == self.type_.is_output() {
            Some(unsafe { &*(&self.parm as *const _ as *const T) })
        } else {
            None
        }
    }

    /// Get mutable reference to value
    pub fn try_mut<T: IsStreamParm>(&mut self) -> Option<&mut T> {
        if T::OUTPUT == self.type_.is_output() {
            Some(unsafe { &mut *(&mut self.parm as *mut _ as *mut T) })
        } else {
            None
        }
    }
}

/// Stream parameter types
pub trait IsStreamParm {
    /// Stream direction
    const OUTPUT: bool;
}

impl IsStreamParm for CaptureParm {
    const OUTPUT: bool = false;
}

impl IsStreamParm for OutputParm {
    const OUTPUT: bool = true;
}

impl core::fmt::Display for StreamParm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.type_.fmt(f)?;
        ": ".fmt(f)?;
        self.try_ref::<CaptureParm>()
            .map(|parm| parm.fmt(f))
            .or_else(|| self.try_ref::<OutputParm>().map(|parm| parm.fmt(f)))
            .unwrap()
    }
}

impl core::fmt::Display for CaptureParm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.time_per_frame.fmt(f)?;
        " #".fmt(f)?;
        self.read_buffers.fmt(f)?;
        ' '.fmt(f)?;
        self.capability.fmt(f)?;
        ','.fmt(f)?;
        self.capture_mode.fmt(f)?;
        " *".fmt(f)?;
        self.extended_mode.fmt(f)
    }
}

impl core::fmt::Display for OutputParm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.time_per_frame.fmt(f)?;
        " #".fmt(f)?;
        self.write_buffers.fmt(f)?;
        ' '.fmt(f)?;
        self.capability.fmt(f)?;
        ','.fmt(f)?;
        self.output_mode.fmt(f)?;
        " *".fmt(f)?;
        self.extended_mode.fmt(f)
    }
}
