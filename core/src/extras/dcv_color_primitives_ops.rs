use crate::FourCc;

impl core::convert::TryFrom<FourCc> for dcv_color_primitives::PixelFormat {
    type Error = FourCc;

    fn try_from(fourcc: FourCc) -> Result<Self, Self::Error> {
        Ok(match fourcc {
            FourCc::Argb32 => Self::Argb,
            FourCc::Bgra32 => Self::Bgra,
            FourCc::Bgr24 => Self::Bgr,
            FourCc::Rgba32 => Self::Rgba,
            FourCc::Rgb24 => Self::Rgb,
            FourCc::Yuv24 => Self::I444,
            FourCc::Yyuv => Self::I422,
            FourCc::Yuv420 => Self::I420,
            FourCc::Nv12 => Self::Nv12,
            _ => return Err(fourcc),
        })
    }
}

impl From<dcv_color_primitives::PixelFormat> for FourCc {
    fn from(pixfmt: dcv_color_primitives::PixelFormat) -> Self {
        use dcv_color_primitives::PixelFormat::*;
        match pixfmt {
            Argb => Self::Argb32,
            Bgra => Self::Bgra32,
            Bgr => Self::Bgr24,
            Rgba => Self::Rgba32,
            Rgb => Self::Rgb24,
            I444 => Self::Yuv24,
            I422 => Self::Yyuv,
            I420 => Self::Yuv420,
            Nv12 => Self::Nv12,
        }
    }
}
