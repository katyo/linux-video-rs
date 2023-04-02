use crate::{types::*, DirectionImpl};

/// Direction types
pub trait Direction: DirectionImpl {
    const IN: bool;
    const OUT: bool;

    fn buffer_type(content: ContentType) -> BufferType;
}

macro_rules! direction_impl {
    ($($(#[$($meta:meta)*])* $type:ident ($is_input:literal) {
        $($(#[$($variant_meta:meta)*])* $content_type:ident = $buffer_type:ident,)*
    })*) => {
        $(
            $(#[$($meta)*])*
            pub struct $type;

            impl Direction for $type {
                const IN: bool = $is_input;
                const OUT: bool = !$is_input;

                fn buffer_type(content: ContentType) -> BufferType {
                    match content {
                        $(
                            ContentType::$content_type => BufferType::$buffer_type,
                        )*
                    }
                }
            }
        )*
    };
}

enum_impl! {
    /// Buffer content type
    enum ContentType {
        Video,
        Vbi,
        SlicedVbi,
        VideoOverlay,
        VideoMplane,
        Sdr,
        Meta,
    }
}

impl ContentType {
    /// All content types
    pub const ALL: [Self; 7] = [
        Self::Video,
        Self::Vbi,
        Self::SlicedVbi,
        Self::VideoOverlay,
        Self::VideoMplane,
        Self::Sdr,
        Self::Meta,
    ];

    /// All video types
    pub const VIDEO: [Self; 3] = [Self::Video, Self::VideoOverlay, Self::VideoMplane];

    /// Is video type
    pub fn is_video(self) -> bool {
        matches!(self, Self::Video | Self::VideoOverlay | Self::VideoMplane)
    }

    /// Get buffer type using direction
    pub fn buffer_type<Dir: Direction>(self) -> BufferType {
        Dir::buffer_type(self)
    }
}

direction_impl! {
    /// Capture (input direction)
    In (true) {
        /// Video capture
        Video = VideoCapture,
        Vbi = VbiCapture,
        SlicedVbi = SlicedVbiCapture,
        VideoOverlay = VideoOverlay,
        VideoMplane = VideoCaptureMplane,
        Sdr = SdrCapture,
        Meta = MetaCapture,
    }

    /// Render (output direction)
    Out (false) {
        Video = VideoOutput,
        Vbi = VbiOutput,
        SlicedVbi = SlicedVbiOutput,
        VideoOverlay = VideoOutputOverlay,
        VideoMplane = VideoOutputMplane,
        Sdr = SdrOutput,
        Meta = MetaOutput,
    }
}

impl BufferType {
    /// All buffer types
    pub const ALL: [Self; 14] = [
        Self::VideoCapture,
        Self::VbiCapture,
        Self::SlicedVbiCapture,
        Self::VideoOverlay,
        Self::VideoCaptureMplane,
        Self::SdrCapture,
        Self::MetaCapture,
        Self::VideoOutput,
        Self::VbiOutput,
        Self::SlicedVbiOutput,
        Self::VideoOutputOverlay,
        Self::VideoOutputMplane,
        Self::SdrOutput,
        Self::MetaOutput,
    ];

    /// Capture buffer types
    pub const CAPTURE: [Self; 7] = [
        Self::VideoCapture,
        Self::VbiCapture,
        Self::SlicedVbiCapture,
        Self::VideoOverlay,
        Self::VideoCaptureMplane,
        Self::SdrCapture,
        Self::MetaCapture,
    ];

    /// Output buffer types
    pub const OUTPUT: [Self; 7] = [
        Self::VideoOutput,
        Self::VbiOutput,
        Self::SlicedVbiOutput,
        Self::VideoOutputOverlay,
        Self::VideoOutputMplane,
        Self::SdrOutput,
        Self::MetaOutput,
    ];

    /// Video buffer types
    pub const VIDEO: [Self; 6] = [
        Self::VideoCapture,
        Self::VideoOverlay,
        Self::VideoCaptureMplane,
        Self::VideoOutput,
        Self::VideoOutputOverlay,
        Self::VideoOutputMplane,
    ];

    /// Vbi buffer types
    pub const VBI: [Self; 4] = [
        Self::VbiCapture,
        Self::SlicedVbiCapture,
        Self::VbiOutput,
        Self::SlicedVbiOutput,
    ];

    /// Sdr buffer types
    pub const SDR: [Self; 2] = [Self::SdrCapture, Self::SdrOutput];

    /// Meta buffer types
    pub const META: [Self; 2] = [Self::MetaCapture, Self::MetaOutput];

    /// Single-planar video buffer types
    pub const VIDEO_SPLANE: [Self; 2] = [Self::VideoCapture, Self::VideoOutput];

    /// Multi-planar video buffer types
    pub const VIDEO_MPLANE: [Self; 2] = [Self::VideoCaptureMplane, Self::VideoOutputMplane];

    /// Overlay video buffer types
    pub const VIDEO_OVERLAY: [Self; 2] = [Self::VideoOverlay, Self::VideoOutputOverlay];

    /// Check that buffer type is supported according to capabilities
    pub fn is_supported(self, capabilities: CapabilityFlag) -> bool {
        match self {
            Self::VideoCapture => capabilities.contains(CapabilityFlag::VideoCapture),
            Self::VbiCapture => capabilities.contains(CapabilityFlag::VbiCapture),
            Self::SlicedVbiCapture => capabilities.contains(CapabilityFlag::SlicedVbiCapture),
            Self::VideoOverlay => capabilities.contains(CapabilityFlag::VideoOverlay),
            Self::VideoCaptureMplane => capabilities.contains(CapabilityFlag::VideoCaptureMplane),
            Self::SdrCapture => capabilities.contains(CapabilityFlag::SdrCapture),
            Self::MetaCapture => capabilities.contains(CapabilityFlag::MetaCapture),
            Self::VideoOutput => capabilities.contains(CapabilityFlag::VideoOutput),
            Self::VbiOutput => capabilities.contains(CapabilityFlag::VbiOutput),
            Self::SlicedVbiOutput => capabilities.contains(CapabilityFlag::SlicedVbiOutput),
            Self::VideoOutputOverlay => capabilities.contains(CapabilityFlag::VideoOutputOverlay),
            Self::VideoOutputMplane => capabilities.contains(CapabilityFlag::VideoOutputMplane),
            Self::SdrOutput => capabilities.contains(CapabilityFlag::SdrOutput),
            Self::MetaOutput => capabilities.contains(CapabilityFlag::MetaOutput),
        }
    }

    /// Get corresponding content type
    pub fn content(&self) -> ContentType {
        match self {
            Self::VideoCapture | Self::VideoOutput => ContentType::Video,
            Self::VbiCapture | Self::VbiOutput => ContentType::Vbi,
            Self::SlicedVbiCapture | Self::SlicedVbiOutput => ContentType::SlicedVbi,
            Self::VideoOverlay | Self::VideoOutputOverlay => ContentType::VideoOverlay,
            Self::VideoCaptureMplane | Self::VideoOutputMplane => ContentType::VideoMplane,
            Self::SdrCapture | Self::SdrOutput => ContentType::Sdr,
            Self::MetaCapture | Self::MetaOutput => ContentType::Meta,
        }
    }
}

impl core::fmt::Display for Rect {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.left.fmt(f)?;
        ','.fmt(f)?;
        self.top.fmt(f)?;
        ' '.fmt(f)?;
        self.width.fmt(f)?;
        'x'.fmt(f)?;
        self.height.fmt(f)
    }
}

impl core::fmt::Display for Fract {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.numerator.fmt(f)?;
        '/'.fmt(f)?;
        self.denominator.fmt(f)
    }
}

impl core::fmt::Display for Area {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.width.fmt(f)?;
        'x'.fmt(f)?;
        self.height.fmt(f)
    }
}

impl core::fmt::Display for TimeCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.type_.fmt(f)?;
        if !self.flags.is_none() {
            ' '.fmt(f)?;
            self.flags.fmt(f)?;
        }
        ' '.fmt(f)?;
        write!(
            f,
            "{:02}:{:02}:{:02},{:03}",
            self.hours, self.minutes, self.seconds, self.frames
        )
    }
}
