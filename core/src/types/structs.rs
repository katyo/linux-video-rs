use super::*;
use getset::{CopyGetters, Setters};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Edid {
    pub(crate) pad: u32,
    pub(crate) start_block: u32,
    pub(crate) blocks: u32,
    pub(crate) reserved: [u32; 5],
    pub(crate) edid: *mut u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct H264Sps {
    pub(crate) profile_idc: u8,
    pub(crate) constraint_set_flags: u8,
    pub(crate) level_idc: u8,
    pub(crate) seq_parameter_set_id: u8,
    pub(crate) chroma_format_idc: u8,
    pub(crate) bit_depth_luma_minus8: u8,
    pub(crate) bit_depth_chroma_minus8: u8,
    pub(crate) log2_max_frame_num_minus4: u8,
    pub(crate) pic_order_cnt_type: u8,
    pub(crate) log2_max_pic_order_cnt_lsb_minus4: u8,
    pub(crate) max_num_ref_frames: u8,
    pub(crate) num_ref_frames_in_pic_order_cnt_cycle: u8,
    pub(crate) offset_for_ref_frame: [i32; 255],
    pub(crate) offset_for_non_ref_pic: i32,
    pub(crate) offset_for_top_to_bottom_field: i32,
    pub(crate) pic_width_in_mbs_minus1: u16,
    pub(crate) pic_height_in_map_units_minus1: u16,
    pub(crate) flags: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct H264Pps {
    pub(crate) pic_parameter_set_id: u8,
    pub(crate) seq_parameter_set_id: u8,
    pub(crate) num_slice_groups_minus1: u8,
    pub(crate) num_ref_idx_l0_default_active_minus1: u8,
    pub(crate) num_ref_idx_l1_default_active_minus1: u8,
    pub(crate) weighted_bipred_idc: u8,
    pub(crate) pic_init_qp_minus26: i8,
    pub(crate) pic_init_qs_minus26: i8,
    pub(crate) chroma_qp_index_offset: i8,
    pub(crate) second_chroma_qp_index_offset: i8,
    pub(crate) flags: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct H264ScalingMatrix {
    pub(crate) scaling_list_4x4: [[u8; 16]; 6],
    pub(crate) scaling_list_8x8: [[u8; 64]; 6],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct H264WeightFactors {
    pub(crate) luma_weight: [i16; 32],
    pub(crate) luma_offset: [i16; 32],
    pub(crate) chroma_weight: [[i16; 2]; 32],
    pub(crate) chroma_offset: [[i16; 2]; 32],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct H264PredWeights {
    pub(crate) luma_log2_weight_denom: u16,
    pub(crate) chroma_log2_weight_denom: u16,
    pub(crate) weight_factors: [H264WeightFactors; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct H264Reference {
    pub(crate) fields: u8,
    pub(crate) index: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct H264SliceParams {
    pub(crate) header_bit_size: u32,
    pub(crate) first_mb_in_slice: u32,
    pub(crate) slice_type: u8,
    pub(crate) colour_plane_id: u8,
    pub(crate) redundant_pic_cnt: u8,
    pub(crate) cabac_init_idc: u8,
    pub(crate) slice_qp_delta: i8,
    pub(crate) slice_qs_delta: i8,
    pub(crate) disable_deblocking_filter_idc: u8,
    pub(crate) slice_alpha_c0_offset_div2: i8,
    pub(crate) slice_beta_offset_div2: i8,
    pub(crate) num_ref_idx_l0_active_minus1: u8,
    pub(crate) num_ref_idx_l1_active_minus1: u8,
    pub(crate) reserved: u8,
    pub(crate) ref_pic_list0: [H264Reference; H264_REF_LIST_LEN],
    pub(crate) ref_pic_list1: [H264Reference; H264_REF_LIST_LEN],
    pub(crate) flags: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct H264DpbEntry {
    pub(crate) reference_ts: u64,
    pub(crate) pic_num: u32,
    pub(crate) frame_num: u16,
    pub(crate) fields: u8,
    pub(crate) reserved: [u8; 5],
    pub(crate) top_field_order_cnt: i32,
    pub(crate) bottom_field_order_cnt: i32,
    pub(crate) flags: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct H264DecodeParams {
    pub(crate) dpb: [H264DpbEntry; H264_NUM_DPB_ENTRIES],
    pub(crate) nal_ref_idc: u16,
    pub(crate) frame_num: u16,
    pub(crate) top_field_order_cnt: i32,
    pub(crate) bottom_field_order_cnt: i32,
    pub(crate) idr_pic_id: u16,
    pub(crate) pic_order_cnt_lsb: u16,
    pub(crate) delta_pic_order_cnt_bottom: i32,
    pub(crate) delta_pic_order_cnt0: i32,
    pub(crate) delta_pic_order_cnt1: i32,
    pub(crate) dec_ref_pic_marking_bit_size: u32,
    pub(crate) pic_order_cnt_bit_size: u32,
    pub(crate) slice_group_change_cycle: u32,
    pub(crate) reserved: u32,
    pub(crate) flags: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FwhtParams {
    pub(crate) backward_ref_ts: u64,
    pub(crate) version: FwhtVersion,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) flags: FwhtFlag,
    pub(crate) colorspace: ColorSpace,
    pub(crate) xfer_func: XferFunc,
    pub(crate) ycbcr_enc: YcbcrEncoding,
    pub(crate) quantization: Quantization,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vp8Segment {
    pub(crate) quant_update: [i8; 4],
    pub(crate) lf_update: [i8; 4],
    pub(crate) segment_probs: [u8; 3],
    pub(crate) padding: u8,
    pub(crate) flags: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vp8LoopFilter {
    pub(crate) ref_frm_delta: [i8; 4],
    pub(crate) mb_mode_delta: [i8; 4],
    pub(crate) sharpness_level: u8,
    pub(crate) level: u8,
    pub(crate) padding: u16,
    pub(crate) flags: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vp8Quantization {
    pub(crate) y_ac_qi: u8,
    pub(crate) y_dc_delta: i8,
    pub(crate) y2_dc_delta: i8,
    pub(crate) y2_ac_delta: i8,
    pub(crate) uv_dc_delta: i8,
    pub(crate) uv_ac_delta: i8,
    pub(crate) padding: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vp8Entropy {
    pub(crate) coeff_probs: [[[[u8; VP8_COEFF_PROB_CNT]; 3]; 8]; 4],
    pub(crate) y_mode_probs: [u8; 4],
    pub(crate) uv_mode_probs: [u8; 3],
    pub(crate) mv_probs: [[u8; VP8_MV_PROB_CNT]; 2],
    pub(crate) padding: [u8; 3],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vp8EntropyCoderState {
    pub(crate) range: u8,
    pub(crate) value: u8,
    pub(crate) bit_count: u8,
    pub(crate) padding: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vp8Frame {
    pub(crate) segment: Vp8Segment,
    pub(crate) lf: Vp8LoopFilter,
    pub(crate) quant: Vp8Quantization,
    pub(crate) entropy: Vp8Entropy,
    pub(crate) coder_state: Vp8EntropyCoderState,
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) horizontal_scale: u8,
    pub(crate) vertical_scale: u8,
    pub(crate) version: u8,
    pub(crate) prob_skip_false: u8,
    pub(crate) prob_intra: u8,
    pub(crate) prob_last: u8,
    pub(crate) prob_gf: u8,
    pub(crate) num_dct_parts: u8,
    pub(crate) first_part_size: u32,
    pub(crate) first_part_header_bits: u32,
    pub(crate) dct_part_sizes: [u32; 8],
    pub(crate) last_frame_ts: u64,
    pub(crate) golden_frame_ts: u64,
    pub(crate) alt_frame_ts: u64,
    pub(crate) flags: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mpeg2Sequence {
    pub(crate) horizontal_size: u16,
    pub(crate) vertical_size: u16,
    pub(crate) vbv_buffer_size: u32,
    pub(crate) profile_and_level_indication: u16,
    pub(crate) chroma_format: u8,
    pub(crate) flags: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mpeg2Picture {
    pub(crate) backward_ref_ts: u64,
    pub(crate) forward_ref_ts: u64,
    pub(crate) flags: u32,
    pub(crate) f_code: [[u8; 2]; 2],
    pub(crate) picture_coding_type: u8,
    pub(crate) picture_structure: u8,
    pub(crate) intra_dc_precision: u8,
    pub(crate) reserved: [u8; 5],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mpeg2Quantisation {
    pub(crate) intra_quantiser_matrix: [u8; 64],
    pub(crate) non_intra_quantiser_matrix: [u8; 64],
    pub(crate) chroma_intra_quantiser_matrix: [u8; 64],
    pub(crate) chroma_non_intra_quantiser_matrix: [u8; 64],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Hdr10CllInfo {
    pub(crate) max_content_light_level: u16,
    pub(crate) max_pic_average_light_level: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Hdr10MasteringDisplay {
    pub(crate) display_primaries_x: [u16; 3],
    pub(crate) display_primaries_y: [u16; 3],
    pub(crate) white_point_x: u16,
    pub(crate) white_point_y: u16,
    pub(crate) max_display_mastering_luminance: u32,
    pub(crate) min_display_mastering_luminance: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vp9LoopFilter {
    pub(crate) ref_deltas: [i8; 4],
    pub(crate) mode_deltas: [i8; 2],
    pub(crate) level: u8,
    pub(crate) sharpness: u8,
    pub(crate) flags: u8,
    pub(crate) reserved: [u8; 7],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vp9Quantization {
    pub(crate) base_q_idx: u8,
    pub(crate) delta_q_y_dc: i8,
    pub(crate) delta_q_uv_dc: i8,
    pub(crate) delta_q_uv_ac: i8,
    pub(crate) reserved: [u8; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vp9Segmentation {
    pub(crate) feature_data: [[i16; 4]; 8],
    pub(crate) feature_enabled: [u8; 8],
    pub(crate) tree_probs: [u8; 7],
    pub(crate) pred_probs: [u8; 3],
    pub(crate) flags: u8,
    pub(crate) reserved: [u8; 5],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vp9Frame {
    pub(crate) lf: Vp9LoopFilter,
    pub(crate) quant: Vp9Quantization,
    pub(crate) seg: Vp9Segmentation,
    pub(crate) flags: u32,
    pub(crate) compressed_header_size: u16,
    pub(crate) uncompressed_header_size: u16,
    pub(crate) frame_width_minus_1: u16,
    pub(crate) frame_height_minus_1: u16,
    pub(crate) render_width_minus_1: u16,
    pub(crate) render_height_minus_1: u16,
    pub(crate) last_frame_ts: u64,
    pub(crate) golden_frame_ts: u64,
    pub(crate) alt_frame_ts: u64,
    pub(crate) ref_frame_sign_bias: u8,
    pub(crate) reset_frame_context: u8,
    pub(crate) frame_context_idx: u8,
    pub(crate) profile: u8,
    pub(crate) bit_depth: u8,
    pub(crate) interpolation_filter: u8,
    pub(crate) tile_cols_log2: u8,
    pub(crate) tile_rows_log2: u8,
    pub(crate) reference_mode: u8,
    pub(crate) reserved: [u8; 7],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vp9MvProbs {
    pub(crate) joint: [u8; 3],
    pub(crate) sign: [u8; 2],
    pub(crate) classes: [[u8; 10]; 2],
    pub(crate) class0_bit: [u8; 2],
    pub(crate) bits: [[u8; 10]; 2],
    pub(crate) class0_fr: [[[u8; 3]; 2]; 2],
    pub(crate) fr: [[u8; 3]; 2],
    pub(crate) class0_hp: [u8; 2],
    pub(crate) hp: [u8; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vp9CompressedHdr {
    pub(crate) tx_mode: u8,
    pub(crate) tx8: [[u8; 1]; 2],
    pub(crate) tx16: [[u8; 2]; 2],
    pub(crate) tx32: [[u8; 3]; 2],
    #[allow(clippy::type_complexity)]
    pub(crate) coef: [[[[[[u8; 3]; 6]; 6]; 2]; 2]; 4],
    pub(crate) skip: [u8; 3],
    pub(crate) inter_mode: [[u8; 3]; 7],
    pub(crate) interp_filter: [[u8; 2]; 4],
    pub(crate) is_inter: [u8; 4],
    pub(crate) comp_mode: [u8; 5],
    pub(crate) single_ref: [[u8; 2]; 5],
    pub(crate) comp_ref: [u8; 5],
    pub(crate) y_mode: [[u8; 9]; 4],
    pub(crate) uv_mode: [[u8; 9]; 10],
    pub(crate) partition: [[u8; 3]; 16],
    pub(crate) mv: Vp9MvProbs,
}

/// Rectangle data
#[repr(C)]
#[derive(Debug, Copy, Clone, CopyGetters, Setters)]
pub struct Rect {
    /// Left coordinate (X)
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) left: i32,

    /// Top coordinate (Y)
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) top: i32,

    /// Width size
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) width: u32,

    /// Height size
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) height: u32,
}

/// Fraction value
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, CopyGetters, Setters)]
pub struct Fract {
    /// Numerator
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) numerator: u32,

    /// Denominator
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) denominator: u32,
}

/// Area data
#[repr(C)]
#[derive(Debug, Copy, Clone, CopyGetters, Setters)]
pub struct Area {
    /// Width size
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) width: u32,

    /// Height size
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) height: u32,
}

/// Version numbers
#[cfg(target_endian = "little")] // FIXME:
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, CopyGetters)]
pub struct VersionTriple {
    pub(crate) reserved: u8,

    /// Major version number
    #[getset(get_copy = "pub")]
    pub(crate) major: u8,

    /// Minor version number
    #[getset(get_copy = "pub")]
    pub(crate) minor: u8,

    /// Patch version number
    #[getset(get_copy = "pub")]
    pub(crate) patch: u8,
}

/// Version numbers
#[cfg(target_endian = "big")] // FIXME:
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, CopyGetters)]
pub struct VersionTriple {
    /// Patch version number
    #[getset(get_copy = "pub")]
    pub(crate) patch: u8,

    /// Minor version number
    #[getset(get_copy = "pub")]
    pub(crate) minor: u8,

    /// Major version number
    #[getset(get_copy = "pub")]
    pub(crate) major: u8,

    pub(crate) reserved: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, CopyGetters)]
pub struct Capability {
    pub(crate) driver: [u8; 16],
    pub(crate) card: [u8; 32],
    pub(crate) bus_info: [u8; 32],
    /// Get driver version
    #[getset(get_copy = "pub")]
    pub(crate) version: VersionTriple,
    /// Get capabilities
    #[getset(get_copy = "pub")]
    pub(crate) capabilities: CapabilityFlag,
    /// Get device capabilities
    #[getset(get_copy = "pub")]
    pub(crate) device_capabilities: CapabilityFlag,
    pub(crate) reserved: [u32; 3],
}

/// Single-planar pixel format
///
/// Applications set width and height to request an image size, drivers return
/// the closest possible values.
/// In case of planar formats the width and height applies to the largest plane.
/// To avoid ambiguities drivers must return values rounded up to a multiple of
/// the scale factor of any smaller planes. For example when the image format
/// is YUV 4:2:0, width and height must be multiples of two.
///
/// For compressed formats that contain the resolution information encoded inside
/// the stream, when fed to a stateful mem2mem decoder, the fields may be zero to
/// rely on the decoder to detect the right values. For more details see
/// Memory-to-Memory Stateful Video Decoder Interface and format descriptions.
///
/// For compressed formats on the capture side of a stateful mem2mem encoder, the
/// ields must be zero, since the coded size is expected to be calculated internally
/// by the encoder itself, based on the OUTPUT side. For more details see
/// Memory-to-Memory Stateful Video Encoder Interface and format descriptions.
///
#[repr(C)]
#[derive(Copy, Clone, CopyGetters, Setters)]
pub struct PixFormat {
    /// Image width in pixels
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) width: u32,

    /// Image height in pixels
    ///
    /// If field is one of [Field::Top], [Field::Bottom] or [Field::Alternate]
    /// then height refers to the number of lines in the field, otherwise it
    /// refers to the number of lines in the frame (which is twice the field
    /// height for interlaced formats).
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) height: u32,

    /// Pixel format
    ///
    /// The pixel format or type of compression, set by the application.
    /// This is a little endian four character code. V4L2 defines standard
    /// RGB formats in RGB Formats, YUV formats in YUV Formats, and reserved
    /// codes in Reserved Image Formats.
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) pixel_format: FourCc,

    /// Field order, from enum [Field].
    ///
    /// Video images are typically interlaced.
    /// Applications can request to capture or output only the top or bottom
    /// field, or both fields interlaced or sequentially stored in one buffer
    /// or alternating in separate buffers. Drivers return the actual field
    /// order selected. For more details on fields see Field Order.
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) field: Field,

    /// Distance in bytes between the leftmost pixels in two adjacent lines.
    ///
    /// Both applications and drivers can set bytes_per_line to request padding bytes
    /// at the end of each line. Drivers however may ignore the value requested by the
    /// application, returning width times bytes per pixel or a larger value required
    /// by the hardware. That implies applications can just set this field to zero to
    /// get a reasonable default.
    ///
    /// Video hardware may access padding bytes, therefore they must reside in accessible
    /// memory. Consider cases where padding bytes after the last line of an image cross
    /// a system page boundary. Input devices may write padding bytes, the value is
    /// undefined. Output devices ignore the contents of padding bytes.
    ///
    /// When the image format is planar the bytesperline value applies to the first plane
    /// and is divided by the same factor as the width field for the other planes.
    /// For example the Cb and Cr planes of a YUV 4:2:0 image have half as many padding
    /// bytes following each line as the Y plane. To avoid ambiguities drivers must return
    /// a bytesperline value rounded up to a multiple of the scale factor.
    ///
    /// For compressed formats the bytesperline value makes no sense. Applications and
    /// drivers must set this to 0 in that case.
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) bytes_per_line: u32,

    /// Image size
    ///
    /// Size in bytes of the buffer to hold a complete image, set by the driver. Usually
    /// this is bytesperline times height. When the image consists of variable length
    /// compressed data this is the number of bytes required by the codec to support
    /// the worst-case compression scenario.
    ///
    /// The driver will set the value for uncompressed images.
    ///
    /// Clients are allowed to set the sizeimage field for variable length compressed data
    /// flagged with [FmtFlag::Compressed], but the driver may
    /// ignore it and set the value itself, or it may modify the provided value based on
    /// alignment requirements or minimum/maximum size requirements. If the client wants
    /// to leave this to the driver, then it should set sizeimage to 0.
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) size_image: u32,

    /// Image colorspace, from enum [ColorSpace].
    ///
    /// This information supplements the pixelformat and must be set by the driver for
    /// capture streams and by the application for output streams, see Colorspaces.
    /// If the application sets the flag [PixFmtFlag::SetCsc] then the application can
    /// set this field for a capture stream to request a specific colorspace for the
    /// captured image data. If the driver cannot handle requested conversion, it will
    /// return another supported colorspace. The driver indicates that colorspace
    /// conversion is supported by setting the flag [FmtFlag::CscColorSpace] in the
    /// corresponding struct [FmtDesc] during enumeration. See [FmtFlag].
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) color_space: ColorSpace,

    /// This field indicates whether the remaining fields of the struct [PixFormat],
    /// also called the extended fields, are valid. When set to V4L2_PIX_FMT_PRIV_MAGIC,
    /// it indicates that the extended fields have been correctly initialized. When set
    /// to any other value it indicates that the extended fields contain undefined values.
    ///
    /// Applications that wish to use the pixel format extended fields must first ensure
    /// that the feature is supported by querying the device for the
    /// [CapabilityFlag::ExtPixFormat] capability. If the capability isn’t set the pixel
    /// format extended fields are not supported and using the extended fields will lead
    /// to undefined results.
    ///
    /// To use the extended fields, applications must set the priv field to
    /// V4L2_PIX_FMT_PRIV_MAGIC, initialize all the extended fields and zero the unused
    /// bytes of the struct v4l2_format raw_data field.
    ///
    /// When the priv field isn’t set to V4L2_PIX_FMT_PRIV_MAGIC drivers must act as if
    /// all the extended fields were set to zero. On return drivers must set the priv
    /// field to V4L2_PIX_FMT_PRIV_MAGIC and all the extended fields to applicable values.
    pub(crate) priv_: u32,

    /// Pixel format flags
    ///
    /// Flags set by the application or driver, see [PixFmtFlag].
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) flags: PixFmtFlag,

    pub(crate) union_: PixFormatUnion,

    /// Quantization range, from enum [Quantization].
    ///
    /// This information supplements the colorspace and must be set by the driver for
    /// capture streams and by the application for output streams, see Colorspaces.
    /// If the application sets the flag [PixFmtFlag::SetCsc] then the application can
    /// set this field for a capture stream to request a specific quantization range for
    /// the captured image data. If the driver cannot handle requested conversion, it will
    /// return another supported quantization. The driver indicates that quantization
    /// conversion is supported by setting the flag [FmtFlag::CscQuantization] in the
    /// corresponding struct [FmtDesc] during enumeration. See [FmtFlag].
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) quantization: Quantization,

    /// Transfer function, from enum [XferFunc].
    ///
    /// This information supplements the colorspace and must be set by the driver for
    /// capture streams and by the application for output streams, see Colorspaces.
    /// If the application sets the flag [PixFmtFlag::SetCsc] then the application
    /// can set this field for a capture stream to request a specific transfer function
    /// for the captured image data. If the driver cannot handle requested conversion,
    /// it will return another supported transfer function. The driver indicates that
    /// [XferFunc] conversion is supported by setting the flag [FmtFlag::CscXferFunc]
    /// in the corresponding struct [FmtDesc] during enumeration. See [FmtFlag].
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) xfer_func: XferFunc,
}

impl From<FourCc> for PixFormat {
    fn from(pixel_format: FourCc) -> Self {
        let color_space = ColorSpace::from(pixel_format);
        PixFormat {
            width: 0,
            height: 0,
            pixel_format,
            field: Field::None,
            bytes_per_line: 0,
            size_image: 0,
            color_space,
            priv_: 0,
            flags: PixFmtFlag::none(),
            union_: PixFormatUnion::new_default(pixel_format, color_space),
            quantization: Quantization::new_default(
                pixel_format.is_rgb() || pixel_format.is_hsv(),
                color_space,
            ),
            xfer_func: color_space.into(),
        }
    }
}

impl PixFormat {
    /// Get Y’CbCr encoding, from enum [YcbcrEncoding]
    ///
    /// This information supplements the colorspace and must be set by the driver for
    /// capture streams and by the application for output streams, see Colorspaces.
    /// If the application sets the flag [PixFmtFlag::SetCsc] then the application can
    /// set this field for a capture stream to request a specific Y’CbCr encoding for
    /// the captured image data. If the driver cannot handle requested conversion, it
    /// will return another supported encoding. This field is ignored for HSV pixel
    /// formats. The driver indicates that [YcbcrEncoding] conversion is supported by
    /// setting the flag [FmtFlag::CscYcbcrEnc] in the corresponding struct [FmtDesc]
    /// during enumeration. See [FmtFlag].
    pub fn ycbcr_enc(&self) -> Option<YcbcrEncoding> {
        if self.pixel_format.is_ycbcr() {
            Some(unsafe { self.union_.ycbcr_enc })
        } else {
            None
        }
    }

    /// Set Y’CbCr encoding, from enum [YcbcrEncoding]
    pub fn set_ycbcr_enc(&mut self, ycbcr_enc: YcbcrEncoding) {
        if self.pixel_format.is_ycbcr() {
            self.union_.ycbcr_enc = ycbcr_enc;
        }
    }

    /// Get HSV encoding, from enum [HsvEncoding]
    ///
    /// This information supplements the colorspace and must be set by the driver for
    /// capture streams and by the application for output streams, see Colorspaces.
    /// If the application sets the flag [PixFmtFlag::SetCsc] then the application can
    /// set this field for a capture stream to request a specific HSV encoding for the
    /// captured image data. If the driver cannot handle requested conversion, it will
    /// return another supported encoding. This field is ignored for non-HSV pixel
    /// formats. The driver indicates that hsv_enc conversion is supported by setting
    /// the flag [FmtFlag::CscYcbcrEnc] in the corresponding struct [FmtDesc] during
    /// enumeration. See [FmtFlag].
    pub fn hsv_enc(&self) -> Option<HsvEncoding> {
        if self.pixel_format.is_hsv() {
            Some(unsafe { self.union_.hsv_enc })
        } else {
            None
        }
    }

    /// Set HSV encoding, from enum [HsvEncoding]
    pub fn set_hsv_enc(&mut self, hsv_enc: HsvEncoding) {
        if self.pixel_format.is_hsv() {
            self.union_.hsv_enc = hsv_enc;
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union PixFormatUnion {
    pub(crate) ycbcr_enc: YcbcrEncoding,
    pub(crate) hsv_enc: HsvEncoding,
}

impl PixFormatUnion {
    fn new_default(pixfmt: FourCc, colorsp: ColorSpace) -> Self {
        if pixfmt.is_hsv() {
            Self {
                hsv_enc: HsvEncoding::E256,
            }
        } else {
            Self {
                ycbcr_enc: colorsp.into(),
            }
        }
    }
}

/// Format description
#[repr(C)]
#[derive(Debug, Copy, Clone, CopyGetters)]
pub struct FmtDesc {
    /// Format index
    #[getset(get_copy = "pub")]
    pub(crate) index: u32,

    /// Format type
    #[getset(get_copy = "pub")]
    pub(crate) type_: BufferType,

    /// Format flags
    #[getset(get_copy = "pub")]
    pub(crate) flags: FmtFlag,

    pub(crate) description: [u8; 32],

    /// Pixel format
    #[getset(get_copy = "pub")]
    pub(crate) pixel_format: FourCc,

    /// Media bus code
    #[getset(get_copy = "pub")]
    pub(crate) mbus_code: u32,

    pub(crate) reserved: [u32; 3],
}

/// Discrete frame size
pub type FrmSizeDiscrete = Area;

/// Stepwise frame sizes
#[repr(C)]
#[derive(Debug, Copy, Clone, CopyGetters)]
pub struct FrmSizeStepwise {
    /// Minimum width in pixels
    #[getset(get_copy = "pub")]
    pub(crate) min_width: u32,

    /// Maximum width in pixels
    #[getset(get_copy = "pub")]
    pub(crate) max_width: u32,

    /// Width step in pixels
    #[getset(get_copy = "pub")]
    pub(crate) step_width: u32,

    /// Minimum height in pixels
    #[getset(get_copy = "pub")]
    pub(crate) min_height: u32,

    /// Maximum height in pixels
    #[getset(get_copy = "pub")]
    pub(crate) max_height: u32,

    /// Height step in pixels
    #[getset(get_copy = "pub")]
    pub(crate) step_height: u32,
}

/// Frame size description
#[repr(C)]
#[derive(Copy, Clone, CopyGetters)]
pub struct FrmSizeEnum {
    /// Frame size index
    #[getset(get_copy = "pub")]
    pub(crate) index: u32,

    /// Requested pixel format
    #[getset(get_copy = "pub")]
    pub(crate) pixel_format: FourCc,

    /// Frame size type
    #[getset(get_copy = "pub")]
    pub(crate) type_: FrmSizeType,

    pub(crate) union_: FrmSizeEnumUnion,

    pub(crate) reserved: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union FrmSizeEnumUnion {
    pub(crate) discrete: FrmSizeDiscrete,
    pub(crate) stepwise: FrmSizeStepwise,
}

/// Discrete frame interval
pub type FrmIvalDiscrete = Fract;

/// Stepwise frame interval
#[repr(C)]
#[derive(Debug, Copy, Clone, CopyGetters)]
pub struct FrmIvalStepwise {
    /// Minimum frame interval
    #[getset(get_copy = "pub")]
    pub(crate) min: Fract,

    /// Maximum frame interval
    #[getset(get_copy = "pub")]
    pub(crate) max: Fract,

    /// Frame interval step
    #[getset(get_copy = "pub")]
    pub(crate) step: Fract,
}

/// Frame interval description
#[repr(C)]
#[derive(Copy, Clone, CopyGetters)]
pub struct FrmIvalEnum {
    /// Frame inteval index
    #[getset(get_copy = "pub")]
    pub(crate) index: u32,

    /// Requested pixel format
    #[getset(get_copy = "pub")]
    pub(crate) pixel_format: FourCc,

    /// Requested width in pixels
    #[getset(get_copy = "pub")]
    pub(crate) width: u32,

    /// Requested height in pixels
    #[getset(get_copy = "pub")]
    pub(crate) height: u32,

    /// Frame interval type
    #[getset(get_copy = "pub")]
    pub(crate) type_: FrmIvalType,

    pub(crate) union_: FrmIvalEnumUnion,

    pub(crate) reserved: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union FrmIvalEnumUnion {
    pub(crate) discrete: FrmIvalDiscrete,
    pub(crate) stepwise: FrmIvalStepwise,
}

/// Time code
#[repr(C)]
#[derive(Debug, Copy, Clone, CopyGetters, Setters)]
pub struct TimeCode {
    /// Type of time code
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) type_: TimeCodeType,

    /// Time code flags
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) flags: TimeCodeFlag,

    /// Number of frames left
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) frames: u8,

    /// Number of seconds left
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) seconds: u8,

    /// Number of minutes left
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) minutes: u8,

    /// Number of hours left
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) hours: u8,

    pub(crate) userbits: [u8; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JpegCompression {
    pub(crate) quality: int,
    pub(crate) app_n: int,
    pub(crate) app_len: int,
    pub(crate) app_data: [char_; 60],
    pub(crate) com_len: int,
    pub(crate) com_data: [char_; 60],
    pub(crate) jpeg_markers: JpegMarker,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RequestBuffers {
    pub(crate) count: u32,
    pub(crate) type_: BufferType,
    pub(crate) memory: Memory,
    pub(crate) capabilities: BufferCapabilityFlag,
    pub(crate) flags: u8,
    pub(crate) reserved: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Plane {
    pub(crate) bytesused: u32,
    pub(crate) length: u32,
    pub(crate) m: PlaneUnion,
    pub(crate) data_offset: u32,
    pub(crate) reserved: [u32; 11],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union PlaneUnion {
    pub(crate) mem_offset: u32,
    pub(crate) userptr: ulong,
    pub(crate) fd: i32,
}

#[repr(C)]
#[derive(Copy, Clone, CopyGetters)]
pub struct Buffer {
    /// Buffer index
    #[getset(get_copy = "pub")]
    pub(crate) index: u32,

    /// Buffer type
    #[getset(get_copy = "pub")]
    pub(crate) type_: BufferType,

    /// Number of bytes in use
    #[getset(get_copy = "pub")]
    pub(crate) bytes_used: u32,

    /// Buffer flags
    #[getset(get_copy = "pub")]
    pub(crate) flags: BufferFlag,

    /// Buffer field
    #[getset(get_copy = "pub")]
    pub(crate) field: Field,

    /// Buffer time
    pub(crate) timestamp: TimeVal,

    pub(crate) timecode: TimeCode,

    /// Buffer sequence
    #[getset(get_copy = "pub")]
    pub(crate) sequence: u32,

    /// Buffer memory
    #[getset(get_copy = "pub")]
    pub(crate) memory: Memory,

    pub(crate) m: BufferM,

    /// Buffer capacity in bytes
    #[getset(get_copy = "pub")]
    pub(crate) length: u32,

    pub(crate) reserved2: u32,

    pub(crate) union_: BufferUnion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union BufferM {
    pub(crate) offset: u32,
    pub(crate) userptr: ulong,
    pub(crate) planes: *mut Plane,
    pub(crate) fd: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union BufferUnion {
    pub(crate) request_fd: i32,
    pub(crate) reserved: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ExportBuffer {
    pub(crate) type_: BufferType,
    pub(crate) index: u32,
    pub(crate) plane: u32,
    pub(crate) flags: u32,
    pub(crate) fd: i32,
    pub(crate) reserved: [u32; 11],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FrameBuffer {
    pub(crate) capability: FrameBufferCapabilityFlag,
    pub(crate) flags: FrameBufferFlag,
    pub(crate) base: *mut void,
    pub(crate) fmt: FrameBufferFmt,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FrameBufferFmt {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) pixel_format: FourCc,
    pub(crate) field: Field,
    pub(crate) bytes_per_line: u32,
    pub(crate) size_image: u32,
    pub(crate) color_space: ColorSpace,
    pub(crate) priv_: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clip {
    pub(crate) c: Rect,
    pub(crate) next: *mut Clip,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, CopyGetters, Setters)]
pub struct Window {
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) w: Rect,

    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) field: Field,

    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) chromakey: u32,

    pub(crate) clips: *mut Clip,
    pub(crate) clip_count: u32,

    pub(crate) bitmap: *mut void,
    pub(crate) global_alpha: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, CopyGetters, Setters)]
pub struct CaptureParm {
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) capability: IoCapabilityFlag,

    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) capture_mode: IoMode,

    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) time_per_frame: Fract,

    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) extended_mode: u32,

    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) read_buffers: u32,

    pub(crate) reserved: [u32; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, CopyGetters, Setters)]
pub struct OutputParm {
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) capability: IoCapabilityFlag,

    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) output_mode: IoMode,

    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) time_per_frame: Fract,

    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) extended_mode: u32,

    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) write_buffers: u32,

    pub(crate) reserved: [u32; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CropCap {
    pub(crate) type_: BufferType,
    pub(crate) bounds: Rect,
    pub(crate) defrect: Rect,
    pub(crate) pixel_aspect: Fract,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Crop {
    pub(crate) type_: BufferType,
    pub(crate) c: Rect,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Selection {
    pub(crate) type_: BufferType,
    pub(crate) target: SelectionTarget,
    pub(crate) flags: SelectionFlag,
    pub(crate) r: Rect,
    pub(crate) reserved: [u32; 9],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Standard {
    pub(crate) index: u32,
    pub(crate) id: StdId,
    pub(crate) name: [u8; 24],
    pub(crate) frame_period: Fract,
    pub(crate) frame_lines: u32,
    pub(crate) reserved: [u32; 4],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct BtTimings {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) interlaced: u32,
    pub(crate) polarities: u32,
    pub(crate) pixel_clock: u64,
    pub(crate) hfront_porch: u32,
    pub(crate) hsync: u32,
    pub(crate) hback_porch: u32,
    pub(crate) vfront_porch: u32,
    pub(crate) vsync: u32,
    pub(crate) vback_porch: u32,
    pub(crate) il_vfront_porch: u32,
    pub(crate) il_vsync: u32,
    pub(crate) il_vback_porch: u32,
    pub(crate) standards: u32,
    pub(crate) flags: DvFlag,
    pub(crate) picture_aspect: Fract,
    pub(crate) cea861_vic: u8,
    pub(crate) hdmi_vic: u8,
    pub(crate) reserved: [u8; 46],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DvTimings {
    pub(crate) type_: DvTimingsType,
    pub(crate) union_: DvTimingsUnion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union DvTimingsUnion {
    pub(crate) bt: BtTimings,
    pub(crate) reserved: [u32; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EnumDvTimings {
    pub(crate) index: u32,
    pub(crate) pad: u32,
    pub(crate) reserved: [u32; 2],
    pub(crate) timings: DvTimings,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct BtTimingsCap {
    pub(crate) min_width: u32,
    pub(crate) max_width: u32,
    pub(crate) min_height: u32,
    pub(crate) max_height: u32,
    pub(crate) min_pixelclock: u64,
    pub(crate) max_pixelclock: u64,
    pub(crate) standards: DvBtStd,
    pub(crate) capabilities: DvBtCapabilityFlag,
    pub(crate) reserved: [u32; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DvTimingsCap {
    pub(crate) type_: DvTimingsType,
    pub(crate) pad: u32,
    pub(crate) reserved: [u32; 2],
    pub(crate) union_: DvTimingsCapUnion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union DvTimingsCapUnion {
    pub(crate) bt: BtTimingsCap,
    pub(crate) raw_data: [u32; 32],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Input {
    pub(crate) index: u32,
    pub(crate) name: [u8; 32],
    pub(crate) type_: InputType,
    pub(crate) audio_set: u32,
    pub(crate) tuner: TunerType,
    pub(crate) std: StdId,
    pub(crate) status: InputStatusFlag,
    pub(crate) capabilities: InputCapabilityFlag,
    pub(crate) reserved: [u32; 3],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Output {
    pub(crate) index: u32,
    pub(crate) name: [u8; 32],
    pub(crate) type_: OutputType,
    pub(crate) audio_set: u32,
    pub(crate) modulator: u32,
    pub(crate) std: StdId,
    pub(crate) capabilities: OutputCapabilityFlag,
    pub(crate) reserved: [u32; 3],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Control {
    pub(crate) id: u32,
    pub(crate) value: i32,
}

#[repr(C, align(4))]
#[derive(Copy, Clone)]
pub struct ExtControl {
    pub(crate) id: u32,
    pub(crate) size: u32,
    pub(crate) reserved2: [u32; 1],
    pub(crate) union_: ExtControlUnion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union ExtControlUnion {
    pub(crate) value: i32,
    pub(crate) value64: i64,
    pub(crate) string: *mut char_,
    pub(crate) p_u8: *mut u8,
    pub(crate) p_u16: *mut u16,
    pub(crate) p_u32: *mut u32,
    pub(crate) p_area: *mut Area,
    pub(crate) p_h264_sps: *mut H264Sps,
    pub(crate) p_h264_pps: *mut H264Pps,
    pub(crate) p_h264_scaling_matrix: *mut H264ScalingMatrix,
    pub(crate) p_h264_pred_weights: *mut H264PredWeights,
    pub(crate) p_h264_slice_params: *mut H264SliceParams,
    pub(crate) p_h264_decode_params: *mut H264DecodeParams,
    pub(crate) p_fwht_params: *mut FwhtParams,
    pub(crate) p_vp8_frame: *mut Vp8Frame,
    pub(crate) p_mpeg2_sequence: *mut Mpeg2Sequence,
    pub(crate) p_mpeg2_picture: *mut Mpeg2Picture,
    pub(crate) p_mpeg2_quantisation: *mut Mpeg2Quantisation,
    pub(crate) p_vp9_compressed_hdr_probs: *mut Vp9CompressedHdr,
    pub(crate) p_vp9_frame: *mut Vp9Frame,
    pub(crate) ptr: *mut void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExtControls {
    pub(crate) union_: ExtControlsUnion,
    pub(crate) count: u32,
    pub(crate) error_idx: u32,
    pub(crate) request_fd: i32,
    pub(crate) reserved: [u32; 1],
    pub(crate) controls: *mut ExtControl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union ExtControlsUnion {
    pub(crate) ctrl_class: CtrlClass,
    pub(crate) which: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, CopyGetters)]
pub struct QueryCtrl {
    /// Control identifier
    #[getset(get_copy = "pub")]
    pub(crate) id: u32,

    /// Control type
    #[getset(get_copy = "pub")]
    pub(crate) type_: CtrlType,

    pub(crate) name: [u8; 32],

    /// Minimum value, inclusive
    ///
    /// This field gives a lower bound for the control.
    #[getset(get_copy = "pub")]
    pub(crate) min: i32,

    /// Maximum value, inclusive
    ///
    /// This field gives an upper bound for the control
    #[getset(get_copy = "pub")]
    pub(crate) max: i32,

    /// This field gives a step size for the control
    #[getset(get_copy = "pub")]
    pub(crate) step: i32,

    /// Default value of control
    ///
    /// The default value of a [CtrlType::Integer], [CtrlType::Boolean], [CtrlType::BitMask], [CtrlType::Menu] or [CtrlType::IntegerMenu] control. Not valid for other types of controls
    #[getset(get_copy = "pub")]
    pub(crate) default: i32,

    /// Control flags
    #[getset(get_copy = "pub")]
    pub(crate) flags: CtrlFlag,

    pub(crate) reserved: [u32; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, CopyGetters)]
pub struct QueryExtCtrl {
    /// Control identifier
    #[getset(get_copy = "pub")]
    pub(crate) id: u32,

    /// Control type
    #[getset(get_copy = "pub")]
    pub(crate) type_: CtrlType,

    pub(crate) name: [u8; 32],

    /// Minimum value, inclusive
    ///
    /// This field gives a lower bound for the control.
    #[getset(get_copy = "pub")]
    pub(crate) min: i64,

    /// Maximum value, inclusive
    ///
    /// This field gives an upper bound for the control
    #[getset(get_copy = "pub")]
    pub(crate) max: i64,

    /// This field gives a step size for the control
    #[getset(get_copy = "pub")]
    pub(crate) step: u64,

    /// Default value of control
    ///
    /// The default value of a [CtrlType::Integer], [CtrlType::Boolean], [CtrlType::BitMask], [CtrlType::Menu] or [CtrlType::IntegerMenu] control. Not valid for other types of controls
    #[getset(get_copy = "pub")]
    pub(crate) default: i64,

    /// Control flags
    #[getset(get_copy = "pub")]
    pub(crate) flags: CtrlFlag,

    /// The size of the value in bytes
    #[getset(get_copy = "pub")]
    pub(crate) elem_size: u32,

    /// The number of elements in the N-dimensional array
    #[getset(get_copy = "pub")]
    pub(crate) elems: u32,

    pub(crate) nr_of_dims: u32,

    pub(crate) dims: [u32; CTRL_MAX_DIMS],

    pub(crate) reserved: [u32; 32],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct QueryMenu {
    pub(crate) id: u32,
    pub(crate) index: u32,
    pub(crate) union_: QueryMenuUnion,
    pub(crate) reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union QueryMenuUnion {
    pub(crate) name: [u8; 32],
    pub(crate) value: i64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tuner {
    pub(crate) index: u32,
    pub(crate) name: [u8; 32],
    pub(crate) type_: TunerType,
    pub(crate) capability: TunerCapabilityFlag,
    pub(crate) rangelow: u32,
    pub(crate) rangehigh: u32,
    pub(crate) rxsubchans: TunerSubFlag,
    pub(crate) audmode: TunerModeFlag,
    pub(crate) signal: i32,
    pub(crate) afc: i32,
    pub(crate) reserved: [u32; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Modulator {
    pub(crate) index: u32,
    pub(crate) name: [u8; 32],
    pub(crate) capability: TunerCapabilityFlag,
    pub(crate) rangelow: u32,
    pub(crate) rangehigh: u32,
    pub(crate) txsubchans: u32,
    pub(crate) type_: u32,
    pub(crate) reserved: [u32; 3],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Frequency {
    pub(crate) tuner: u32,
    pub(crate) type_: TunerType,
    pub(crate) frequency: u32,
    pub(crate) reserved: [u32; 8],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FrequencyBand {
    pub(crate) tuner: u32,
    pub(crate) type_: TunerType,
    pub(crate) index: u32,
    pub(crate) capability: u32,
    pub(crate) rangelow: u32,
    pub(crate) rangehigh: u32,
    pub(crate) modulation: u32,
    pub(crate) reserved: [u32; 9],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HwFreqSeek {
    pub(crate) tuner: u32,
    pub(crate) type_: TunerType,
    pub(crate) seek_upward: u32,
    pub(crate) wrap_around: u32,
    pub(crate) spacing: u32,
    pub(crate) rangelow: u32,
    pub(crate) rangehigh: u32,
    pub(crate) reserved: [u32; 5],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct RdsData {
    pub(crate) lsb: u8,
    pub(crate) msb: u8,
    pub(crate) block: RdsBlockFlag,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Audio {
    pub(crate) index: u32,
    pub(crate) name: [u8; 32],
    pub(crate) capability: AudioCapabilityFlag,
    pub(crate) mode: AudioMode,
    pub(crate) reserved: [u32; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioOut {
    pub(crate) index: u32,
    pub(crate) name: [u8; 32],
    pub(crate) capability: AudioCapabilityFlag,
    pub(crate) mode: AudioMode,
    pub(crate) reserved: [u32; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EncIdxEntry {
    pub(crate) offset: u64,
    pub(crate) pts: u64,
    pub(crate) length: u32,
    pub(crate) flags: u32,
    pub(crate) reserved: [u32; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EncIdx {
    pub(crate) entries: u32,
    pub(crate) entries_cap: u32,
    pub(crate) reserved: [u32; 4],
    pub(crate) entry: [EncIdxEntry; ENC_IDX_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EncoderCmd {
    pub(crate) cmd: EncCmd,
    pub(crate) flags: EncCmdFlag,
    pub(crate) union_: EncoderCmdUnion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union EncoderCmdUnion {
    pub(crate) raw: EncoderCmdUnionRaw,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EncoderCmdUnionRaw {
    pub(crate) data: [u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DecoderCmd {
    pub(crate) cmd: DecCmd,
    pub(crate) flags: DecCmdFlag,
    pub(crate) union_: DecoderCmdUnion,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union DecoderCmdUnion {
    pub(crate) stop: DecoderCmdUnionStop,
    pub(crate) start: DecoderCmdUnionStart,
    pub(crate) raw: DecoderCmdUnionRaw,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DecoderCmdUnionStop {
    pub(crate) pts: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DecoderCmdUnionStart {
    pub(crate) speed: i32,
    pub(crate) format: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DecoderCmdUnionRaw {
    pub(crate) data: [u32; 16],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, CopyGetters, Setters)]
pub struct VbiFormat {
    pub(crate) sampling_rate: u32,
    pub(crate) offset: u32,
    pub(crate) samples_per_line: u32,
    pub(crate) sample_format: u32,
    pub(crate) start: [i32; 2],
    pub(crate) count: [u32; 2],
    pub(crate) flags: VbiFlags,
    pub(crate) reserved: [u32; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SlicedVbiFormat {
    pub(crate) service_set: u16,
    pub(crate) service_lines: [[u16; 24]; 2],
    pub(crate) io_size: u32,
    pub(crate) reserved: [u32; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SlicedVbiCap {
    pub(crate) service_set: u16,
    pub(crate) service_lines: [[u16; 24]; 2],
    pub(crate) type_: BufferType,
    pub(crate) reserved: [u32; 3],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SlicedVbiData {
    pub(crate) id: SlicedVbiType,
    pub(crate) field: u32,
    pub(crate) line: u32,
    pub(crate) reserved: u32,
    pub(crate) data: [u8; 48],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct MpegVbiItv0Line {
    pub(crate) id: MpegVbiIvtvType,
    pub(crate) data: [u8; 42],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct MpegVbiItv0 {
    pub(crate) linemask: [u32; 2],
    pub(crate) line: [MpegVbiItv0Line; 35],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct MpegVbiITV0 {
    pub(crate) line: [MpegVbiItv0Line; 36],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MpegVbiFmtIvtv {
    pub(crate) magic: [u8; 4],
    pub(crate) union_: MpegVbiFmtIvtvUnion,
}

impl MpegVbiFmtIvtv {
    pub const IVTV_MAGIC0: [u8; 4] = *b"itv0";
    pub const IVTV_MAGIC1: [u8; 4] = *b"ITV0";

    pub fn itv0(&self) -> Option<&MpegVbiItv0> {
        if self.magic == Self::IVTV_MAGIC0 {
            Some(unsafe { &self.union_.itv0 })
        } else {
            None
        }
    }

    pub fn itv0_mut(&mut self) -> Option<&mut MpegVbiItv0> {
        if self.magic == Self::IVTV_MAGIC0 {
            Some(unsafe { &mut self.union_.itv0 })
        } else {
            None
        }
    }

    #[allow(non_snake_case)]
    pub fn ITV0(&self) -> Option<&MpegVbiITV0> {
        if self.magic == Self::IVTV_MAGIC1 {
            Some(unsafe { &self.union_.ITV0 })
        } else {
            None
        }
    }

    #[allow(non_snake_case)]
    pub fn ITV0_mut(&mut self) -> Option<&mut MpegVbiITV0> {
        if self.magic == Self::IVTV_MAGIC1 {
            Some(unsafe { &mut self.union_.ITV0 })
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub(crate) union MpegVbiFmtIvtvUnion {
    pub(crate) itv0: MpegVbiItv0,
    pub(crate) ITV0: MpegVbiITV0,
}

/// Per-plane format definition
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, CopyGetters, Setters)]
pub struct PlanePixFormat {
    /// Image size
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) size_image: u32,

    /// Bytes per line
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) bytes_per_line: u32,

    pub(crate) reserved: [u16; 6],
}

/// Multiplanar format definition
#[repr(C, packed)]
#[derive(Copy, Clone, CopyGetters, Setters)]
pub struct PixFormatMplane {
    /// Width in pixels
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) width: u32,

    /// Height in pixels
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) height: u32,

    /// Pixel format
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) pixel_format: FourCc,

    /// Format field
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) field: Field,

    /// Color space
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) color_space: ColorSpace,

    pub(crate) plane_fmt: [PlanePixFormat; VIDEO_MAX_PLANES],

    pub(crate) num_planes: u8,

    /// Pixel format flags
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) flags: PixFmtFlag,

    pub(crate) union_: PixFormatUnion,

    /// Quantization
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) quantization: Quantization,

    /// Transfer function
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) xfer_func: XferFunc,

    pub(crate) reserved: [u8; 7],
}

/// SDR format definition
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, CopyGetters, Setters)]
pub struct SdrFormat {
    /// Pixel format
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) pixel_format: FourCc,

    /// Buffer size
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) buffer_size: u32,

    pub(crate) reserved: [u8; 24],
}

/// Metadata format definition
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, CopyGetters, Setters)]
pub struct MetaFormat {
    /// Data format
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) data_format: FourCc,

    /// Buffer size
    #[getset(get_copy = "pub", set = "pub")]
    pub(crate) buffer_size: u32,
}

/// Stream data format
#[repr(C)]
#[derive(Copy, Clone, CopyGetters)]
pub struct Format {
    /// Buffer size
    #[getset(get_copy = "pub")]
    pub(crate) type_: BufferType,

    pub(crate) fmt: FormatUnion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union FormatUnion {
    pub(crate) pix: PixFormat,
    pub(crate) pix_mp: PixFormatMplane,
    pub(crate) win: Window,
    pub(crate) vbi: VbiFormat,
    pub(crate) sliced: SlicedVbiFormat,
    pub(crate) sdr: SdrFormat,
    pub(crate) meta: MetaFormat,
    pub(crate) raw_data: [u8; 200],
}

#[repr(C)]
#[derive(Copy, Clone, CopyGetters)]
pub struct StreamParm {
    /// Buffer type
    #[getset(get_copy = "pub")]
    pub(crate) type_: BufferType,
    pub(crate) parm: StreamParmUnion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union StreamParmUnion {
    pub(crate) capture: CaptureParm,
    pub(crate) output: OutputParm,
    pub(crate) raw_data: [u8; 200],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct EventVsync {
    pub(crate) field: Field,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EventCtrl {
    pub(crate) changes: EventCtrlChangeFlag,
    pub(crate) type_: EventType,
    pub(crate) union_: EventCtrlUnion,
    pub(crate) flags: CtrlFlag,
    pub(crate) minimum: i32,
    pub(crate) maximum: i32,
    pub(crate) step: i32,
    pub(crate) default_value: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union EventCtrlUnion {
    pub(crate) value: i32,
    pub(crate) value64: i64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EventFrameSync {
    pub(crate) frame_sequence: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EventSrcChange {
    pub(crate) changes: EventSrcChangeFlag,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EventMotionDet {
    pub(crate) flags: EventMotionDetFlag,
    pub(crate) frame_sequence: u32,
    pub(crate) region_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone, CopyGetters)]
pub struct Event {
    /// Event type
    #[getset(get_copy = "pub")]
    pub(crate) type_: EventType,
    pub(crate) u: EventUnion,
    pub(crate) pending: u32,
    pub(crate) sequence: u32,
    pub(crate) timestamp: TimeSpec,

    /// Event type
    #[getset(get_copy = "pub")]
    pub(crate) id: u32,
    pub(crate) reserved: [u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union EventUnion {
    pub(crate) vsync: EventVsync,
    pub(crate) ctrl: EventCtrl,
    pub(crate) frame_sync: EventFrameSync,
    pub(crate) src_change: EventSrcChange,
    pub(crate) motion_det: EventMotionDet,
    pub(crate) data: [u8; 64],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EventSubscription {
    pub(crate) type_: u32,
    pub(crate) id: u32,
    pub(crate) flags: u32,
    pub(crate) reserved: [u32; 5],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DbgMatch {
    pub(crate) type_: u32,
    pub(crate) union_: DbgMatchUnion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union DbgMatchUnion {
    pub(crate) addr: u32,
    pub(crate) name: [char_; 32],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DbgRegister {
    pub(crate) match_: DbgMatch,
    pub(crate) size: u32,
    pub(crate) reg: u64,
    pub(crate) val: u64,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DbgChipInfo {
    pub(crate) match_: DbgMatch,
    pub(crate) name: [char_; 32],
    pub(crate) flags: u32,
    pub(crate) reserved: [u32; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CreateBuffers {
    pub(crate) index: u32,
    pub(crate) count: u32,
    pub(crate) memory: u32,
    pub(crate) format: Format,
    pub(crate) capabilities: BufferCapabilityFlag,
    pub(crate) flags: BufferFlag,
    pub(crate) reserved: [u32; 6],
}
