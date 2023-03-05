use crate::{calls, types::*, Internal, Result};
use std::os::unix::io::RawFd;

impl Internal<&mut Format> {
    pub fn get(&mut self, fd: RawFd) -> Result<()> {
        unsafe_call!(calls::g_fmt(fd, *self.as_mut() as *mut _)).map(|_| ())
    }
}

impl Internal<&mut Format> {
    pub fn set(&self, fd: RawFd) -> Result<()> {
        unsafe_call!(calls::s_fmt(fd, *self.as_ref() as *const _ as *mut _)).map(|_| ())
    }

    pub fn try_(&self, fd: RawFd) -> Result<()> {
        unsafe_call!(calls::try_fmt(fd, *self.as_ref() as *const _ as *mut _)).map(|_| ())
    }
}

impl From<BufferType> for Format {
    fn from(type_: BufferType) -> Self {
        Self {
            type_,
            fmt: FormatUnion { raw_data: [0; 200] },
        }
    }
}

unsafe impl Send for Format {}

impl Format {
    /// Create from value
    pub fn new<T: IsFormatData>(type_: BufferType, data: T) -> Option<Self> {
        if T::TYPES.contains(&type_) {
            let mut this = Self::from(type_);
            *this.try_mut().unwrap() = data;
            Some(this)
        } else {
            None
        }
    }

    /// Get reference to value
    pub fn try_ref<T: IsFormatData>(&self) -> Option<&T> {
        if T::TYPES.contains(&self.type_()) {
            Some(unsafe { &*(&self.fmt as *const _ as *const T) })
        } else {
            None
        }
    }

    /// Get mutable reference to value
    pub fn try_mut<T: IsFormatData>(&mut self) -> Option<&mut T> {
        if T::TYPES.contains(&self.type_()) {
            Some(unsafe { &mut *(&mut self.fmt as *mut _ as *mut T) })
        } else {
            None
        }
    }
}

/// Format data types
pub trait IsFormatData {
    /// Buffer types which corresponds to format type
    const TYPES: &'static [BufferType];
}

macro_rules! format_impl {
    ($($type:ty: $($buf_type:ident)*,)*) => {
        $(
            impl IsFormatData for $type {
                const TYPES: &'static [BufferType] = &[ $(BufferType::$buf_type,)* ];
            }
        )*

        impl core::fmt::Display for Format {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                self.type_.fmt(f)?;
                ": ".fmt(f)?;
                match self.type_ {
                    $(
                        $(BufferType::$buf_type)|* => self.try_ref::<$type>()
                            .ok_or_else(Default::default)?.fmt(f),
                    )*
                }
            }
        }
    };
}

format_impl! {
    PixFormat: VideoCapture VideoOutput,
    VbiFormat: VbiCapture VbiOutput,
    SlicedVbiFormat: SlicedVbiCapture SlicedVbiOutput,
    Window: VideoOverlay VideoOutputOverlay,
    PixFormatMplane: VideoCaptureMplane VideoOutputMplane,
    SdrFormat: SdrCapture SdrOutput,
    MetaFormat: MetaCapture MetaOutput,
}

impl core::fmt::Display for PixFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.width.fmt(f)?;
        'x'.fmt(f)?;
        self.height.fmt(f)?;
        ' '.fmt(f)?;
        self.pixel_format.fmt(f)?;
        ' '.fmt(f)?;
        self.field.fmt(f)?;
        " #".fmt(f)?;
        self.bytes_per_line.fmt(f)?;
        '/'.fmt(f)?;
        self.size_image.fmt(f)?;
        ' '.fmt(f)?;
        self.color_space.fmt(f)?;
        ' '.fmt(f)?;
        // TODO: union
        self.quantization.fmt(f)?;
        ' '.fmt(f)?;
        self.xfer_func.fmt(f)
    }
}

impl PixFormatMplane {
    /// Plane formats
    pub fn plane_fmt(&self) -> &[PlanePixFormat] {
        &self.plane_fmt[..self.num_planes as usize]
    }

    /// Plane formats
    pub fn plane_fmt_mut(&mut self) -> &mut [PlanePixFormat] {
        &mut self.plane_fmt[..self.num_planes as usize]
    }
}

impl core::fmt::Display for PixFormatMplane {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.width().fmt(f)?;
        'x'.fmt(f)?;
        self.height().fmt(f)?;
        ' '.fmt(f)?;
        self.pixel_format().fmt(f)?;
        ' '.fmt(f)?;
        self.field().fmt(f)?;
        for plane in self.plane_fmt() {
            " #".fmt(f)?;
            plane.bytes_per_line().fmt(f)?;
            '/'.fmt(f)?;
            plane.size_image().fmt(f)?;
        }
        ' '.fmt(f)?;
        self.color_space().fmt(f)?;
        ' '.fmt(f)?;
        // TODO: union
        self.quantization().fmt(f)?;
        ' '.fmt(f)?;
        self.xfer_func().fmt(f)
    }
}

impl core::fmt::Display for SdrFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.pixel_format().fmt(f)?;
        " #".fmt(f)?;
        self.buffer_size().fmt(f)
    }
}

impl core::fmt::Display for MetaFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.data_format().fmt(f)?;
        " #".fmt(f)?;
        self.buffer_size().fmt(f)
    }
}

impl core::fmt::Display for Window {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        // TODO:
        self.w.fmt(f)?;
        ' '.fmt(f)?;
        self.field.fmt(f)?;
        " !".fmt(f)?;
        self.chromakey.fmt(f)
    }
}

impl core::fmt::Display for VbiFormat {
    fn fmt(&self, _f: &mut core::fmt::Formatter) -> core::fmt::Result {
        todo!()
    }
}

impl core::fmt::Display for SlicedVbiFormat {
    fn fmt(&self, _f: &mut core::fmt::Formatter) -> core::fmt::Result {
        todo!()
    }
}
