pub const H264_NUM_DPB_ENTRIES: usize = 16;
pub const H264_REF_LIST_LEN: usize = 2 * H264_NUM_DPB_ENTRIES;

pub const VP8_COEFF_PROB_CNT: usize = 11;
pub const VP8_MV_PROB_CNT: usize = 19;

pub const VP9_SEGMENT_FEATURE_ENABLED_MASK: u32 = 15;

pub const VP9_PROFILE_MAX: u32 = 3;
pub const VP9_NUM_FRAME_CTX: u32 = 4;

pub const CTRL_CLASS_MPEG: u32 = 10027008;

pub const PIX_FMT_PRIV_MAGIC: u32 = 4276996862;

pub const CTRL_ID_MASK: u32 = 268435455;
pub const CTRL_MAX_DIMS: usize = 4;
pub const CTRL_WHICH_CUR_VAL: u32 = 0;
pub const CTRL_WHICH_DEF_VAL: u32 = 251658240;
pub const CTRL_WHICH_REQUEST_VAL: u32 = 251723776;

pub const CID_MAX_CTRLS: usize = 1024;
pub const CID_PRIVATE_BASE: u32 = 134217728;

pub const ENC_IDX_ENTRIES: usize = 64;

pub const VBI_ITU_525_F1_START: u32 = 1;
pub const VBI_ITU_525_F2_START: u32 = 264;
pub const VBI_ITU_625_F1_START: u32 = 1;
pub const VBI_ITU_625_F2_START: u32 = 314;

pub const VIDEO_MAX_FRAME: usize = 32;
pub const VIDEO_MAX_PLANES: usize = 8;
