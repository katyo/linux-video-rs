use crate::{consts::*, enums::*, fourcc::*, stdid::*};
pub(crate) use nix::sys::time::{TimeSpec, TimeVal};
pub(crate) use std::os::raw::{c_int as int, c_ulong as ulong, c_void as void};
pub(crate) use u8 as char_; // To unify all C-strings

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub(crate) left: i32,
    pub(crate) top: i32,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fract {
    pub(crate) numerator: u32,
    pub(crate) denominator: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Area {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

#[cfg(target_endian = "little")] // FIXME:
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct VersionTriple {
    pub(crate) reserved: u8,
    pub(crate) major: u8,
    pub(crate) minor: u8,
    pub(crate) patch: u8,
}

#[cfg(target_endian = "big")] // FIXME:
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct VersionTriple {
    pub(crate) patch: u8,
    pub(crate) minor: u8,
    pub(crate) major: u8,
    pub(crate) reserved: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Capability {
    pub(crate) driver: [u8; 16],
    pub(crate) card: [u8; 32],
    pub(crate) bus_info: [u8; 32],
    pub(crate) version: VersionTriple,
    pub(crate) capabilities: CapabilityFlag,
    pub(crate) device_caps: CapabilityFlag,
    pub(crate) reserved: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PixFormat {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) pixelformat: FourCc,
    pub(crate) field: Field,
    pub(crate) bytesperline: u32,
    pub(crate) sizeimage: u32,
    pub(crate) colorspace: ColorSpace,
    pub(crate) priv_: u32,
    pub(crate) flags: PixFmtFlag,
    pub(crate) union_: PixFormatUnion,
    pub(crate) quantization: Quantization,
    pub(crate) xfer_func: XferFunc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union PixFormatUnion {
    pub(crate) ycbcr_enc: YcbcrEncoding,
    pub(crate) hsv_enc: HsvEncoding,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FmtDesc {
    pub(crate) index: u32,
    pub(crate) type_: BufferType,
    pub(crate) flags: FmtFlag,
    pub(crate) description: [u8; 32],
    pub(crate) pixelformat: FourCc,
    pub(crate) mbus_code: u32,
    pub(crate) reserved: [u32; 3],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FrmSizeDiscrete {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FrmSizeStepwise {
    pub(crate) min_width: u32,
    pub(crate) max_width: u32,
    pub(crate) step_width: u32,
    pub(crate) min_height: u32,
    pub(crate) max_height: u32,
    pub(crate) step_height: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FrmSizeEnum {
    pub(crate) index: u32,
    pub(crate) pixel_format: u32,
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FrmIvalStepwise {
    pub(crate) min: Fract,
    pub(crate) max: Fract,
    pub(crate) step: Fract,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FrmIvalEnum {
    pub(crate) index: u32,
    pub(crate) pixel_format: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) type_: FrmIvalType,
    pub(crate) union_: FrmIvalEnumUnion,
    pub(crate) reserved: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union FrmIvalEnumUnion {
    pub(crate) discrete: Fract,
    pub(crate) stepwise: FrmIvalStepwise,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TimeCode {
    pub(crate) type_: TimeCodeType,
    pub(crate) flags: TimeCodeFlag,
    pub(crate) frames: u8,
    pub(crate) seconds: u8,
    pub(crate) minutes: u8,
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
    pub(crate) memory: u32,
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
#[derive(Copy, Clone)]
pub struct Buffer {
    pub(crate) index: u32,
    pub(crate) type_: BufferType,
    pub(crate) bytesused: u32,
    pub(crate) flags: BufferFlag,
    pub(crate) field: Field,
    pub(crate) timestamp: TimeVal,
    pub(crate) timecode: TimeCode,
    pub(crate) sequence: u32,
    pub(crate) memory: u32,
    pub(crate) m: BufferM,
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
    pub(crate) pixelformat: FourCc,
    pub(crate) field: Field,
    pub(crate) bytesperline: u32,
    pub(crate) sizeimage: u32,
    pub(crate) colorspace: ColorSpace,
    pub(crate) priv_: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clip {
    pub(crate) c: Rect,
    pub(crate) next: *mut Clip,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Window {
    pub(crate) w: Rect,
    pub(crate) field: u32,
    pub(crate) chromakey: u32,
    pub(crate) clips: *mut Clip,
    pub(crate) clipcount: u32,
    pub(crate) bitmap: *mut void,
    pub(crate) global_alpha: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CaptureParm {
    pub(crate) capability: CaptureCapabilityFlag,
    pub(crate) capturemode: CaptureModeFlag,
    pub(crate) timeperframe: Fract,
    pub(crate) extendedmode: u32,
    pub(crate) readbuffers: u32,
    pub(crate) reserved: [u32; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OutputParm {
    pub(crate) capability: u32,
    pub(crate) outputmode: u32,
    pub(crate) timeperframe: Fract,
    pub(crate) extendedmode: u32,
    pub(crate) writebuffers: u32,
    pub(crate) reserved: [u32; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CropCap {
    pub(crate) type_: BufferType,
    pub(crate) bounds: Rect,
    pub(crate) defrect: Rect,
    pub(crate) pixelaspect: Fract,
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
    pub(crate) frameperiod: Fract,
    pub(crate) framelines: u32,
    pub(crate) reserved: [u32; 4],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct BtTimings {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) interlaced: u32,
    pub(crate) polarities: u32,
    pub(crate) pixelclock: u64,
    pub(crate) hfrontporch: u32,
    pub(crate) hsync: u32,
    pub(crate) hbackporch: u32,
    pub(crate) vfrontporch: u32,
    pub(crate) vsync: u32,
    pub(crate) vbackporch: u32,
    pub(crate) il_vfrontporch: u32,
    pub(crate) il_vsync: u32,
    pub(crate) il_vbackporch: u32,
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
    pub(crate) audioset: u32,
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
    pub(crate) audioset: u32,
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

#[repr(C, packed)]
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
#[derive(Debug, Copy, Clone)]
pub struct QueryCtrl {
    pub(crate) id: u32,
    pub(crate) type_: CtrlType,
    pub(crate) name: [u8; 32],
    pub(crate) minimum: i32,
    pub(crate) maximum: i32,
    pub(crate) step: i32,
    pub(crate) default_value: i32,
    pub(crate) flags: CtrlFlag,
    pub(crate) reserved: [u32; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct QueryExtCtrl {
    pub(crate) id: u32,
    pub(crate) type_: CtrlType,
    pub(crate) name: [u8; 32],
    pub(crate) minimum: i64,
    pub(crate) maximum: i64,
    pub(crate) step: u64,
    pub(crate) default_value: i64,
    pub(crate) flags: CtrlFlag,
    pub(crate) elem_size: u32,
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
#[derive(Debug, Copy, Clone)]
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

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct PlanePixFormat {
    pub(crate) sizeimage: u32,
    pub(crate) bytesperline: u32,
    pub(crate) reserved: [u16; 6],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PixFormatMplane {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) pixelformat: FourCc,
    pub(crate) field: Field,
    pub(crate) colorspace: ColorSpace,
    pub(crate) plane_fmt: [PlanePixFormat; 8],
    pub(crate) num_planes: u8,
    pub(crate) flags: PixFmtFlag,
    pub(crate) union_: PixFormatUnion,
    pub(crate) quantization: Quantization,
    pub(crate) xfer_func: XferFunc,
    pub(crate) reserved: [u8; 7],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SdrFormat {
    pub(crate) pixelformat: FourCc,
    pub(crate) buffersize: u32,
    pub(crate) reserved: [u8; 24],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct MetaFormat {
    pub(crate) dataformat: u32,
    pub(crate) buffersize: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Format {
    pub(crate) type_: u32,
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
#[derive(Copy, Clone)]
pub struct StreamParm {
    pub(crate) type_: u32,
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
    pub(crate) field: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EventCtrl {
    pub(crate) changes: EventCtrlChangeFlag,
    pub(crate) type_: u32,
    pub(crate) union_: EventCtrlUnion,
    pub(crate) flags: u32,
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
#[derive(Copy, Clone)]
pub struct Event {
    pub(crate) type_: EventType,
    pub(crate) u: EventUnion,
    pub(crate) pending: u32,
    pub(crate) sequence: u32,
    pub(crate) timestamp: TimeSpec,
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

value_impl! {
    i32: Integer,
    bool: Boolean,
    i64: Integer64,
    CtrlClass: CtrlClass,
    u8: U8 BitMask Menu IntegerMenu,
    u16: U16 BitMask Menu IntegerMenu,
    u32: U32 BitMask Menu IntegerMenu,
    Area: Area,
    Hdr10CllInfo: Hdr10CllInfo,
    Hdr10MasteringDisplay: Hdr10MasteringDisplay,
    H264Sps: H264Sps,
    H264Pps: H264Pps,
    H264ScalingMatrix: H264ScalingMatrix,
    H264SliceParams: H264SliceParams,
    H264DecodeParams: H264DecodeParams,
    H264PredWeights: H264PredWeights,
    FwhtParams: FwhtParams,
    //Vp8Params: Vp8Params,
    Mpeg2Quantisation: Mpeg2Quantisation,
    Mpeg2Sequence: Mpeg2Sequence,
    Mpeg2Picture: Mpeg2Picture,
    Vp9CompressedHdr: Vp9CompressedHdr,
    Vp9Frame: Vp9Frame,
}
