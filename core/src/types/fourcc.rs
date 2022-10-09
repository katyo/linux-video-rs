macro_rules! fourcc_impl {
    ($($(#[$($meta:meta)*])* $name:ident = $a:literal $b:literal $c:literal $d:literal $($opt:ident)*,)*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(u32)]
        pub enum FourCc {
            $($(#[$($meta)*])* $name = fourcc_impl!(@$($opt)*: $a $b $c $d),)*
        }

        impl core::fmt::Display for FourCc {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                let raw = *self as u32;
                (raw as u8 as char).fmt(f)?;
                ((raw >> 8) as u8 as char).fmt(f)?;
                ((raw >> 16) as u8 as char).fmt(f)?;
                ((raw >> 24) as u8 as char).fmt(f)
            }
        }
    };

    (@: $a:literal $b:literal $c:literal $d:literal) => {
        ($a as u32) | (($b as u32) << 8) | (($c as u32) << 16) | (($d as u32) << 24)
    };

    (@be: $a:literal $b:literal $c:literal $d:literal) => {
        fourcc_impl!(@: $a $b $c $d) | (1 << 31)
    };
}

fourcc_impl! {
    /* RGB formats (1 or 2 bytes per pixel) */
    /// 8  RGB-3-3-2
    Rgb332 = 'R' 'G' 'B' '1',
    /// 16  xxxxrrrr ggggbbbb
    Rgb444 = 'R' '4' '4' '4',
    /// 16  aaaarrrr ggggbbbb
    Argb444 = 'A' 'R' '1' '2',
    /// 16  xxxxrrrr ggggbbbb
    Xrgb444 = 'X' 'R' '1' '2',
    /// 16  rrrrgggg bbbbaaaa
    Rgba444 = 'R' 'A' '1' '2',
    /// 16  rrrrgggg bbbbxxxx
    Rgbx444 = 'R' 'X' '1' '2',
    /// 16  aaaabbbb ggggrrrr
    Abgr444 = 'A' 'B' '1' '2',
    /// 16  xxxxbbbb ggggrrrr
    Xbgr444 = 'X' 'B' '1' '2',
    /// 16  bbbbgggg rrrraaaa
    Bgra444 = 'G' 'A' '1' '2',
    /// 16  bbbbgggg rrrrxxxx
    Bgrx444 = 'B' 'X' '1' '2',
    /// 16  RGB-5-5-5
    Rgb555 = 'R' 'G' 'B' 'O',
    /// 16  ARGB-1-5-5-5
    Argb555 = 'A' 'R' '1' '5',
    /// 16  XRGB-1-5-5-5
    Xrgb555 = 'X' 'R' '1' '5',
    /// 16  RGBA-5-5-5-1
    Rgba555 = 'R' 'A' '1' '5',
    /// 16  RGBX-5-5-5-1
    Rgbx555 = 'R' 'X' '1' '5',
    /// 16  ABGR-1-5-5-5
    Abgr555 = 'A' 'B' '1' '5',
    /// 16  XBGR-1-5-5-5
    Xbgr555 = 'X' 'B' '1' '5',
    /// 16  BGRA-5-5-5-1
    Bgra555 = 'B' 'A' '1' '5',
    /// 16  BGRX-5-5-5-1
    Bgrx555 = 'B' 'X' '1' '5',
    /// 16  RGB-5-6-5
    Rgb565 = 'R' 'G' 'B' 'P',
    /// 16  RGB-5-5-5 BE
    Rgb555x = 'R' 'G' 'B' 'Q',
    /// 16  ARGB-5-5-5 BE
    Argb555x = 'A' 'R' '1' '5' be,
    /// 16  XRGB-5-5-5 BE
    Xrgb555x = 'X' 'R' '1' '5' be,
    /// 16  RGB-5-6-5 BE
    Rgb565x = 'R' 'G' 'B' 'R',

    /* RGB formats (3 or 4 bytes per pixel) */
    /// 18  BGR-6-6-6
    Bgr666 = 'B' 'G' 'R' 'H',
    /// 24  BGR-8-8-8
    Bgr24 = 'B' 'G' 'R' '3',
    /// 24  RGB-8-8-8
    Rgb24 = 'R' 'G' 'B' '3',
    /// 32  BGR-8-8-8-8
    Bgr32 = 'B' 'G' 'R' '4',
    /// 32  BGRA-8-8-8-8
    Abgr32 = 'A' 'R' '2' '4',
    /// 32  BGRX-8-8-8-8
    Xbgr32 = 'X' 'R' '2' '4',
    /// 32  ABGR-8-8-8-8
    Bgra32 = 'R' 'A' '2' '4',
    /// 32  XBGR-8-8-8-8
    Bgrx32 = 'R' 'X' '2' '4',
    /// 32  RGB-8-8-8-8
    Rgb32 = 'R' 'G' 'B' '4',
    /// 32  RGBA-8-8-8-8
    Rgba32 = 'A' 'B' '2' '4',
    /// 32  RGBX-8-8-8-8
    Rgbx32 = 'X' 'B' '2' '4',
    /// 32  ARGB-8-8-8-8
    Argb32 = 'B' 'A' '2' '4',
    /// 32  XRGB-8-8-8-8
    Xrgb32 = 'B' 'X' '2' '4',

    /* Grey formats */
    /// 8  Greyscale
    Grey = 'G' 'R' 'E' 'Y',
    /// 4  Greyscale
    Y4 = 'Y' '0' '4' ' ',
    /// 6  Greyscale
    Y6 = 'Y' '0' '6' ' ',
    /// 10  Greyscale
    Y10 = 'Y' '1' '0' ' ',
    /// 12  Greyscale
    Y12 = 'Y' '1' '2' ' ',
    /// 14  Greyscale
    Y14 = 'Y' '1' '4' ' ',
    /// 16  Greyscale
    Y16 = 'Y' '1' '6' ' ',
    /// 16  Greyscale BE
    Y16be = 'Y' '1' '6' ' ' be,

    /* Grey bit-packed formats */
    /// 10  Greyscale bit-packed
    Y10bpack = 'Y' '1' '0' 'B',
    /// 10  Greyscale, MIPI RAW10 packed
    Y10p = 'Y' '1' '0' 'P',
    /// IPU3 packed 10-bit greyscale
    Ipu3Y10 = 'i' 'p' '3' 'y',

    /* Palette formats */
    /// 8  8-bit palette
    Pal8 = 'P' 'A' 'L' '8',

    /* Chrominance formats */
    /// 8  UV 4:4
    Uv8 = 'U' 'V' '8' ' ',

    /* Luminance+Chrominance formats */
    /// 16  YUV 4:2:2
    Yuyv = 'Y' 'U' 'Y' 'V',
    /// 16  YUV 4:2:2
    Yyuv = 'Y' 'Y' 'U' 'V',
    /// 16 YVU 4:2:2
    Yvyu = 'Y' 'V' 'Y' 'U',
    /// 16  YUV 4:2:2
    Uyvy = 'U' 'Y' 'V' 'Y',
    /// 16  YUV 4:2:2
    Vyuy = 'V' 'Y' 'U' 'Y',
    /// 12  YUV 4:1:1
    Y41p = 'Y' '4' '1' 'P',
    /// 16  xxxxyyyy uuuuvvvv
    Yuv444 = 'Y' '4' '4' '4',
    /// 16  YUV-5-5-5
    Yuv555 = 'Y' 'U' 'V' 'O',
    /// 16  YUV-5-6-5
    Yuv565 = 'Y' 'U' 'V' 'P',
    /// 24  YUV-8-8-8
    Yuv24 = 'Y' 'U' 'V' '3',
    /// 32  YUV-8-8-8-8
    Yuv32 = 'Y' 'U' 'V' '4',
    /// 32  AYUV-8-8-8-8
    Ayuv32 = 'A' 'Y' 'U' 'V',
    /// 32  XYUV-8-8-8-8
    Xyuv32 = 'X' 'Y' 'U' 'V',
    /// 32  VUYA-8-8-8-8
    Vuya32 = 'V' 'U' 'Y' 'A',
    /// 32  VUYX-8-8-8-8
    Vuyx32 = 'V' 'U' 'Y' 'X',
    /// 12  YUV 4:2:0 2 lines y, 1 line uv interleaved
    M420 = 'M' '4' '2' '0',

    /* two planes -- one Y, one Cr + Cb interleaved  */
    /// 12  Y/CbCr 4:2:0
    Nv12 = 'N' 'V' '1' '2',
    /// 12  Y/CrCb 4:2:0
    Nv21 = 'N' 'V' '2' '1',
    /// 16  Y/CbCr 4:2:2
    Nv16 = 'N' 'V' '1' '6',
    /// 16  Y/CrCb 4:2:2
    Nv61 = 'N' 'V' '6' '1',
    /// 24  Y/CbCr 4:4:4
    Nv24 = 'N' 'V' '2' '4',
    /// 24  Y/CrCb 4:4:4
    Nv42 = 'N' 'V' '4' '2',

    /* two non contiguous planes - one Y, one Cr + Cb interleaved  */
    /// 12  Y/CbCr 4:2:0
    Nv12m = 'N' 'M' '1' '2',
    /// 21  Y/CrCb 4:2:0
    Nv21m = 'N' 'M' '2' '1',
    /// 16  Y/CbCr 4:2:2
    Nv16m = 'N' 'M' '1' '6',
    /// 16  Y/CrCb 4:2:2
    Nv61m = 'N' 'M' '6' '1',

    /* three planes - Y Cb, Cr */
    /// 9  YUV 4:1:0
    Yuv410 = 'Y' 'U' 'V' '9',
    /// 9  YVU 4:1:0
    Yvu410 = 'Y' 'V' 'U' '9',
    /// 12  YVU411 planar
    Yuv411p = '4' '1' '1' 'P',
    /// 12  YUV 4:2:0
    Yuv420 = 'Y' 'U' '1' '2',
    /// 12  YVU 4:2:0
    Yvu420 = 'Y' 'V' '1' '2',
    /// 16  YVU422 planar
    Yuv422p = '4' '2' '2' 'P',

    /* three non contiguous planes - Y, Cb, Cr */
    /// 12  YUV420 planar
    Yuv420m = 'Y' 'M' '1' '2',
    /// 12  YVU420 planar
    Yvu420m = 'Y' 'M' '2' '1',
    /// 16  YUV422 planar
    Yuv422m = 'Y' 'M' '1' '6',
    /// 16  YVU422 planar
    Yvu422m = 'Y' 'M' '6' '1',
    /// 24  YUV444 planar
    Yuv444m = 'Y' 'M' '2' '4',
    /// 24  YVU444 planar
    Yvu444m = 'Y' 'M' '4' '2',

    /* Tiled YUV formats */
    /// 12  Y/CbCr 4:2:0  4x4 tiles
    Nv12_4l4 = 'V' 'T' '1' '2',
    /// 12  Y/CbCr 4:2:0 16x16 tiles
    Nv12_16l16 = 'H' 'M' '1' '2',
    /// 12  Y/CbCr 4:2:0 32x32 tiles
    Nv12_32l32 = 'S' 'T' '1' '2',

    /* Tiled YUV formats, non contiguous planes */
    /// 12  Y/CbCr 4:2:0 64x32 tiles
    Nv12mt = 'T' 'M' '1' '2',
    /// 12  Y/CbCr 4:2:0 16x16 tiles
    Nv12mt16x16 = 'V' 'M' '1' '2',
    /// Y/CbCr 4:2:0 8x128 tiles
    Nv12m8l128 = 'N' 'A' '1' '2',
    /// Y/CbCr 4:2:0 10-bit 8x128 tiles
    Nv12m10be8l128 = 'N' 'T' '1' '2' be,

    /* Bayer formats - see http://www.siliconimaging.com/RGB%20Bayer.htm */
    /// 8  BGBG.. GRGR..
    Sbggr8 = 'B' 'A' '8' '1',
    /// 8  GBGB.. RGRG..
    Sgbrg8 = 'G' 'B' 'R' 'G',
    /// 8  GRGR.. BGBG..
    Sgrbg8 = 'G' 'R' 'B' 'G',
    /// 8  RGRG.. GBGB..
    Srggb8 = 'R' 'G' 'G' 'B',
    /// 10  BGBG.. GRGR..
    Sbggr10 = 'B' 'G' '1' '0',
    /// 10  GBGB.. RGRG..
    Sgbrg10 = 'G' 'B' '1' '0',
    /// 10  GRGR.. BGBG..
    Sgrbg10 = 'B' 'A' '1' '0',
    /// 10  RGRG.. GBGB..
    Srggb10 = 'R' 'G' '1' '0',
    /* 10bit raw bayer packed, 5 bytes for every 4 pixels */
    ///
    Sbggr10p = 'p' 'B' 'A' 'A',
    ///
    Sgbrg10p = 'p' 'G' 'A' 'A',
    ///
    Sgrbg10p = 'p' 'g' 'A' 'A',
    ///
    Srggb10p = 'p' 'R' 'A' 'A',
    /* 10bit raw bayer a-law compressed to 8 bits */
    ///
    Sbggr10alaw8 = 'a' 'B' 'A' '8',
    ///
    Sgbrg10alaw8 = 'a' 'G' 'A' '8',
    ///
    Sgrbg10alaw8 = 'a' 'g' 'A' '8',
    ///
    Srggb10alaw8 = 'a' 'R' 'A' '8',
    /* 10bit raw bayer DPCM compressed to 8 bits */
    ///
    Sbggr10dpcm8 = 'b' 'B' 'A' '8',
    ///
    Sgbrg10dpcm8 = 'b' 'G' 'A' '8',
    ///
    Sgrbg10dpcm8 = 'B' 'D' '1' '0',
    ///
    Srggb10dpcm8 = 'b' 'R' 'A' '8',
    /// 12  BGBG.. GRGR..
    Sbggr12 = 'B' 'G' '1' '2',
    /// 12  GBGB.. RGRG..
    Sgbrg12 = 'G' 'B' '1' '2',
    /// 12  GRGR.. BGBG..
    Sgrbg12 = 'B' 'A' '1' '2',
    /// 12  RGRG.. GBGB..
    Srggb12 = 'R' 'G' '1' '2',
    /* 12bit raw bayer packed, 6 bytes for every 4 pixels */
    ///
    Sbggr12p = 'p' 'B' 'C' 'C',
    ///
    Sgbrg12p = 'p' 'G' 'C' 'C',
    ///
    Sgrbg12p = 'p' 'g' 'C' 'C',
    ///
    Srggb12p = 'p' 'R' 'C' 'C',
    /// 14  BGBG.. GRGR..
    Sbggr14 = 'B' 'G' '1' '4',
    /// 14  GBGB.. RGRG..
    Sgbrg14 = 'G' 'B' '1' '4',
    /// 14  GRGR.. BGBG..
    Sgrbg14 = 'G' 'R' '1' '4',
    /// 14  RGRG.. GBGB..
    Srggb14 = 'R' 'G' '1' '4',
    /* 14bit raw bayer packed, 7 bytes for every 4 pixels */
    ///
    Sbggr14p = 'p' 'B' 'E' 'E',
    ///
    Sgbrg14p = 'p' 'G' 'E' 'E',
    ///
    Sgrbg14p = 'p' 'g' 'E' 'E',
    ///
    Srggb14p = 'p' 'R' 'E' 'E',
    /// 16  BGBG.. GRGR..
    Sbggr16 = 'B' 'Y' 'R' '2',
    /// 16  GBGB.. RGRG..
    Sgbrg16 = 'G' 'B' '1' '6',
    /// 16  GRGR.. BGBG..
    Sgrbg16 = 'G' 'R' '1' '6',
    /// 16  RGRG.. GBGB..
    Srggb16 = 'R' 'G' '1' '6',

    /* HSV formats */
    ///
    Hsv24 = 'H' 'S' 'V' '3',
    ///
    Hsv32 = 'H' 'S' 'V' '4',

    /* compressed formats */
    /// Motion-JPEG
    Mjpeg = 'M' 'J' 'P' 'G',
    /// JFIF JPEG
    Jpeg = 'J' 'P' 'E' 'G',
    /// 1394
    Dv = 'd' 'v' 's' 'd',
    /// MPEG-1/2/4 Multiplexed
    Mpeg = 'M' 'P' 'E' 'G',
    /// H264 with start codes
    H264 = 'H' '2' '6' '4',
    /// H264 without start codes
    H264NoSc = 'A' 'V' 'C' '1',
    /// H264 MVC
    H264Mvc = 'M' '2' '6' '4',
    /// H263
    H263 = 'H' '2' '6' '3',
    /// MPEG-1 ES
    Mpeg1 = 'M' 'P' 'G' '1',
    /// MPEG-2 ES
    Mpeg2 = 'M' 'P' 'G' '2',
    /// MPEG-2 parsed slice data
    Mpeg2Slice = 'M' 'G' '2' 'S',
    /// MPEG-4 part 2 ES
    Mpeg4 = 'M' 'P' 'G' '4',
    /// Xvid
    Xvid = 'X' 'V' 'I' 'D',
    /// SMPTE 421M Annex G compliant stream
    Vc1AnnexG = 'V' 'C' '1' 'G',
    /// SMPTE 421M Annex L compliant stream
    Vc1AnnexL = 'V' 'C' '1' 'L',
    /// VP8
    Vp8 = 'V' 'P' '8' '0',
    /// VP8 parsed frame
    Vp8Frame = 'V' 'P' '8' 'F',
    /// VP9
    Vp9 = 'V' 'P' '9' '0',
    /// VP9 parsed frame
    Vp9Frame = 'V' 'P' '9' 'F',
    /// HEVC aka H.265
    Hevc = 'H' 'E' 'V' 'C',
    /// Fast Walsh Hadamard Transform (vicodec)
    Fwht = 'F' 'W' 'H' 'T',
    /// Stateless FWHT (vicodec)
    FwhtStateless = 'S' 'F' 'W' 'H',
    /// H264 parsed slices
    H264Slice = 'S' '2' '6' '4',

    /*  Vendor-specific formats   */
    /// cpia1 YUV
    Cpia1 = 'C' 'P' 'I' 'A',
    /// Winnov hw compress
    Wnva = 'W' 'N' 'V' 'A',
    /// SN9C10x compression
    Sn9c10x = 'S' '9' '1' '0',
    /// SN9C20x YUV 4:2:0
    Sn9c20xI420 = 'S' '9' '2' '0',
    /// pwc older webcam
    Pwc1 = 'P' 'W' 'C' '1',
    /// pwc newer webcam
    Pwc2 = 'P' 'W' 'C' '2',
    /// ET61X251 compression
    Et61x251 = 'E' '6' '2' '5',
    /// YUYV per line
    Spca501 = 'S' '5' '0' '1',
    /// YYUV per line
    Spca505 = 'S' '5' '0' '5',
    /// YUVY per line
    Spca508 = 'S' '5' '0' '8',
    /// compressed GBRG bayer
    Spca561 = 'S' '5' '6' '1',
    /// compressed BGGR bayer
    Pac207 = 'P' '2' '0' '7',
    /// compressed BGGR bayer
    Mr97310a = 'M' '3' '1' '0',
    /// compressed RGGB bayer
    Jl2005bcd = 'J' 'L' '2' '0',
    /// compressed GBRG bayer
    Sn9c2028 = 'S' 'O' 'N' 'X',
    /// compressed RGGB bayer
    Sq905c = '9' '0' '5' 'C',
    /// Pixart 73xx JPEG
    Pjpg = 'P' 'J' 'P' 'G',
    /// ov511 JPEG
    Ov511 = 'O' '5' '1' '1',
    /// ov518 JPEG
    Ov518 = 'O' '5' '1' '8',
    /// stv0680 bayer
    Stv0680 = 'S' '6' '8' '0',
    /// tm5600/tm60x0
    Tm6000 = 'T' 'M' '6' '0',
    /// one line of Y then 1 line of VYUY
    CitYyvyuy = 'C' 'I' 'T' 'V',
    /// YUV420 planar in blocks of 256 pixels
    Konica420 = 'K' 'O' 'N' 'I',
    /// JPEG-Lite
    Jpgl = 'J' 'P' 'G' 'L',
    /// se401 janggu compressed rgb
    Se401 = 'S' '4' '0' '1',
    /// S5C73M3 interleaved UYVY/JPEG
    S5cUyvyJpg = 'S' '5' 'C' 'I',
    /// Greyscale 8-bit L/R interleaved
    Y8i = 'Y' '8' 'I' ' ',
    /// Greyscale 12-bit L/R interleaved
    Y12i = 'Y' '1' '2' 'I',
    /// Depth data 16-bit
    Z16 = 'Z' '1' '6' ' ',
    /// Mediatek compressed block mode
    Mt21c = 'M' 'T' '2' '1',
    /// Mediatek 8-bit block mode, two non-contiguous planes
    Mm21 = 'M' 'M' '2' '1',
    /// Intel Planar Greyscale 10-bit and Depth 16-bit
    Inzi = 'I' 'N' 'Z' 'I',
    /// Intel 4-bit packed depth confidence information
    Cnf4 = 'C' 'N' 'F' '4',
    /// BTTV 8-bit dithered RGB
    Hi240 = 'H' 'I' '2' '4',
    /// Qualcomm 8-bit compressed
    Qc08c = 'Q' '0' '8' 'C',
    /// Qualcomm 10-bit compressed
    Qc10c = 'Q' '1' '0' 'C',

    /* 10bit raw packed, 32 bytes for every 25 pixels, last LSB 6 bits unused */
    /// IPU3 packed 10-bit BGGR bayer
    Ipu3Sbggr10 = 'i' 'p' '3' 'b',
    /// IPU3 packed 10-bit GBRG bayer
    Ipu3Sgbrg10 = 'i' 'p' '3' 'g',
    /// IPU3 packed 10-bit GRBG bayer
    Ipu3Sgrbg10 = 'i' 'p' '3' 'G',
    /// IPU3 packed 10-bit RGGB bayer
    Ipu3Srggb10 = 'i' 'p' '3' 'r',

    /* SDR formats - used only for Software Defined Radio devices */
    /// IQ u8
    SdrCu8 = 'C' 'U' '0' '8',
    /// IQ u16le
    SdrCu16le = 'C' 'U' '1' '6',
    /// complex s8
    SdrCs8 = 'C' 'S' '0' '8',
    /// complex s14le
    SdrCs14le = 'C' 'S' '1' '4',
    /// real u12le
    SdrRu12le = 'R' 'U' '1' '2',
    /// planar complex u16be
    SdrPcu16be = 'P' 'C' '1' '6',
    /// planar complex u18be
    SdrPcu18be = 'P' 'C' '1' '8',
    /// planar complex u20be
    SdrPcu20be = 'P' 'C' '2' '0',

    /* Touch formats - used for Touch devices */
    /// 16-bit signed deltas
    TchDeltaTd16 = 'T' 'D' '1' '6',
    /// 8-bit signed deltas
    TchDeltaTd08 = 'T' 'D' '0' '8',
    /// 16-bit unsigned touch data
    TchTu16 = 'T' 'U' '1' '6',
    /// 8-bit unsigned touch data
    TchTu08 = 'T' 'U' '0' '8',

    /* Meta-data formats */
    /// R-Car VSP1 1-D Histogram
    MetaVsp1Hgo = 'V' 'S' 'P' 'H',
    /// R-Car VSP1 2-D Histogram
    MetaVsp1Hgt = 'V' 'S' 'P' 'T',
    /// UVC Payload Header metadata
    MetaUvc = 'U' 'V' 'C' 'H',
    /// D4XX Payload Header metadata
    MetaD4xx = 'D' '4' 'X' 'X',
    /// Vivid Metadata
    MetaVivid = 'V' 'I' 'V' 'D',

    /* Vendor specific - used for RK_ISP1 camera sub-system */
    /// Rockchip ISP1 3A Parameters
    MetaRkIsp1Params = 'R' 'K' '1' 'P',
    /// Rockchip ISP1 3A Statistics
    MetaRkIsp1Stat3a = 'R' 'K' '1' 'S',
}
