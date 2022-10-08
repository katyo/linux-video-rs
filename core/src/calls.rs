use crate::types::*;
use nix::{ioctl_none, ioctl_read, ioctl_readwrite, ioctl_write_ptr};

const MAGIC: u8 = b'V';

ioctl_read!(query_cap, MAGIC, 0, Capability);
ioctl_readwrite!(enum_fmt, MAGIC, 2, FmtDesc);
ioctl_readwrite!(g_fmt, MAGIC, 4, Format);
ioctl_readwrite!(s_fmt, MAGIC, 5, Format);
ioctl_readwrite!(req_bufs, MAGIC, 8, RequestBuffers);
ioctl_readwrite!(query_buf, MAGIC, 9, Buffer);
ioctl_read!(g_fbuf, MAGIC, 10, FrameBuffer);
ioctl_write_ptr!(s_fbuf, MAGIC, 11, FrameBuffer);
ioctl_write_ptr!(overlay, MAGIC, 14, int);
ioctl_readwrite!(q_buf, MAGIC, 15, Buffer);
ioctl_readwrite!(exp_buf, MAGIC, 16, ExportBuffer);
ioctl_readwrite!(dq_buf, MAGIC, 17, Buffer);
ioctl_write_ptr!(stream_on, MAGIC, 18, int);
ioctl_write_ptr!(stream_off, MAGIC, 19, int);
ioctl_readwrite!(g_parm, MAGIC, 21, StreamParm);
ioctl_readwrite!(s_parm, MAGIC, 22, StreamParm);
ioctl_read!(g_std, MAGIC, 23, StdId);
ioctl_write_ptr!(s_std, MAGIC, 24, StdId);
ioctl_readwrite!(enum_std, MAGIC, 25, Standard);
ioctl_readwrite!(enum_input, MAGIC, 26, Input);
ioctl_readwrite!(g_ctrl, MAGIC, 27, Control);
ioctl_readwrite!(s_ctrl, MAGIC, 28, Control);
ioctl_readwrite!(g_tuner, MAGIC, 29, Tuner);
ioctl_write_ptr!(s_tuner, MAGIC, 30, Tuner);
ioctl_read!(g_audio, MAGIC, 33, Audio);
ioctl_write_ptr!(s_audio, MAGIC, 34, Audio);
ioctl_readwrite!(query_ctrl, MAGIC, 36, QueryCtrl);
ioctl_readwrite!(query_menu, MAGIC, 37, QueryMenu);
ioctl_read!(g_input, MAGIC, 38, int);
ioctl_readwrite!(s_input, MAGIC, 39, int);
ioctl_readwrite!(g_edid, MAGIC, 40, Edid);
ioctl_readwrite!(s_edid, MAGIC, 41, Edid);
ioctl_read!(g_output, MAGIC, 46, int);
ioctl_readwrite!(s_output, MAGIC, 47, int);
ioctl_readwrite!(enum_output, MAGIC, 48, Output);
ioctl_read!(g_audio_out, MAGIC, 49, AudioOut);
ioctl_write_ptr!(s_audio_out, MAGIC, 50, AudioOut);
ioctl_readwrite!(g_modulator, MAGIC, 54, Modulator);
ioctl_write_ptr!(s_modulator, MAGIC, 55, Modulator);
ioctl_readwrite!(g_frequency, MAGIC, 56, Frequency);
ioctl_write_ptr!(s_frequency, MAGIC, 57, Frequency);
ioctl_readwrite!(crop_cap, MAGIC, 58, CropCap);
ioctl_readwrite!(g_crop, MAGIC, 59, Crop);
ioctl_write_ptr!(s_crop, MAGIC, 60, Crop);
ioctl_read!(g_jpeg_comp, MAGIC, 61, JpegCompression);
ioctl_write_ptr!(s_jpeg_comp, MAGIC, 62, JpegCompression);
ioctl_read!(query_std, MAGIC, 63, StdId);
ioctl_readwrite!(try_fmt, MAGIC, 64, Format);
ioctl_readwrite!(enum_audio, MAGIC, 65, Audio);
ioctl_readwrite!(enum_audio_out, MAGIC, 66, AudioOut);
ioctl_read!(g_priority, MAGIC, 67, Priority);
ioctl_write_ptr!(s_priority, MAGIC, 68, Priority);
ioctl_readwrite!(g_sliced_vbi_cap, MAGIC, 69, SlicedVbiCap);
ioctl_none!(log_status, MAGIC, 70);
ioctl_readwrite!(g_ext_ctrls, MAGIC, 71, ExtControls);
ioctl_readwrite!(s_ext_ctrls, MAGIC, 72, ExtControls);
ioctl_readwrite!(try_ext_ctrls, MAGIC, 73, ExtControls);
ioctl_readwrite!(enum_frame_sizes, MAGIC, 74, FrmSizeEnum);
ioctl_readwrite!(enum_frame_intervals, MAGIC, 75, FrmIvalEnum);
ioctl_read!(g_enc_index, MAGIC, 76, EncIdx);
ioctl_readwrite!(encoder_cmd, MAGIC, 77, EncoderCmd);
ioctl_readwrite!(try_encoder_cmd, MAGIC, 78, EncoderCmd);

/*
 * Experimental, meant for debugging, testing and internal use.
 * Only implemented if CONFIG_VIDEO_ADV_DEBUG is defined.
 * You must be root to use these ioctls. Never use these in applications!
 */
ioctl_write_ptr!(dbg_s_register, MAGIC, 79, DbgRegister);
ioctl_readwrite!(dbg_g_register, MAGIC, 80, DbgRegister);

ioctl_write_ptr!(s_hw_freq_seek, MAGIC, 82, HwFreqSeek);
ioctl_readwrite!(s_dv_timings, MAGIC, 87, DvTimings);
ioctl_readwrite!(g_dv_timings, MAGIC, 88, DvTimings);
ioctl_read!(dq_event, MAGIC, 89, Event);
ioctl_write_ptr!(subscribe_event, MAGIC, 90, EventSubscription);
ioctl_write_ptr!(unsubscribe_event, MAGIC, 91, EventSubscription);
ioctl_readwrite!(create_bufs, MAGIC, 92, CreateBuffers);
ioctl_readwrite!(prepare_buf, MAGIC, 93, Buffer);
ioctl_readwrite!(g_selection, MAGIC, 94, Selection);
ioctl_readwrite!(s_selection, MAGIC, 95, Selection);
ioctl_readwrite!(decoder_cmd, MAGIC, 96, DecoderCmd);
ioctl_readwrite!(try_decoder_cmd, MAGIC, 97, DecoderCmd);
ioctl_readwrite!(enum_dv_timings, MAGIC, 98, EnumDvTimings);
ioctl_read!(query_dv_timings, MAGIC, 99, DvTimings);
ioctl_readwrite!(dv_timings_cap, MAGIC, 100, DvTimingsCap);
ioctl_readwrite!(enum_freq_bands, MAGIC, 101, FrequencyBand);

/*
 * Experimental, meant for debugging, testing and internal use.
 * Never use this in applications!
 */
ioctl_readwrite!(dbg_g_chip_info, MAGIC, 102, DbgChipInfo);

ioctl_readwrite!(query_ext_ctrl, MAGIC, 103, QueryExtCtrl);
