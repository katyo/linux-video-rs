enum_impl! {
    enum SelectionTarget {
        Crop = 0x0,
        CropDefault = 0x1,
        CropBounds = 0x2,
        NativeSize = 0x3,
        Compose = 0x100,
        ComposeDefault = 0x101,
        ComposeBounds = 0x102,
        ComposePadded = 0x103,
        //CropActive = 0x0,
        //ComposeActive = 0x100,
    }

    mask SelectionFlag {
        Ge = 0x1,
        Le = 0x2,
        KeepConfig = 0x4,
    }

    mask Lock {
        Exposure = 0x1,
        WhiteBalance = 0x2,
        Focus = 0x4,
    }

    mask AutoFocusStatus {
        Idle = 0x0,
        Busy = 0x1,
        Reached = 0x2,
        Failed = 0x4,
    }

    enum CameraOrientation {
        Front,
        Back,
        External,
    }

    mask FlashFault {
        OverVoltage = 0x1,
        Timeout = 0x2,
        OverTemperature = 0x4,
        ShortCircuit = 0x8,
        OverCurrent = 0x10,
        Indicator = 0x20,
        UnderVoltage = 0x40,
        InputVoltage = 0x80,
        LedOverTemperature = 0x100,
    }

    enum JpegActiveMarker {
        App0 = 0x1,
        App1 = 0x2,
        Com = 0x10000,
        Dqt = 0x20000,
        Dht = 0x40000,
    }

    mask H264SpsConstraintFlag {
        Set0 = 0x1,
        Set1 = 0x2,
        Set2 = 0x4,
        Set3 = 0x8,
        Set4 = 0x10,
        Set5 = 0x20,
    }

    mask H264SpsFlag {
        SeparateColourPlane = 0x1,
        QpprimeYZeroTransformBypass = 0x2,
        DeltaPicOrderAlwaysZero = 0x4,
        GapsInFrameNumValueAllowed = 0x8,
        FrameMbsOnly = 0x10,
        MbAdaptiveFrameField = 0x20,
        Direct8x8Inference = 0x40,
    }

    mask H264PpsFlag {
        EntropyCodingMode = 0x1,
        BottomFieldPicOrderInFramePresent = 0x2,
        WeightedPred = 0x4,
        DeblockingFilterControlPresent = 0x8,
        ConstrainedIntraPred = 0x10,
        RedundantPicCntPresent = 0x20,
        Transform8x8Mode = 0x40,
        ScalingMatrixPresent = 0x80,
    }

    enum H264SliceType {
        P,
        B,
        I,
        Sp,
        Si,
    }

    mask H264SliceFlag {
        DirectSpatialMvPred = 0x1,
        SpForSwitch = 0x2,
    }

    enum H264Ref {
        TopField = 1,
        BottomField = 2,
        Frame = 3,
    }

    mask H264DpbEntryFlag {
        Valid = 0x1,
        Active = 0x2,
        LongTerm = 0x4,
        Field = 0x8,
    }

    mask H264DecodeParamFlag {
        IdrPic = 0x1,
        FieldPic = 0x2,
        BottomField = 0x4,
    }

    mask Vp8SegmentFlag {
        Enabled = 0x1,
        UpdateMap = 0x2,
        UpdateFeatureData = 0x4,
        DeltaValueMode = 0x8,
    }

    mask Vp8LoopFilter {
        AdjEnable = 0x1,
        DeltaUpdate = 0x2,
        FilterTypeSimple = 0x4,
    }

    mask Vp8FrameFlag {
        KeyFrame = 0x1,
        Experimental = 0x2,
        ShowFrame = 0x4,
        MbNoSkipCoeff = 0x8,
        SignBiasGolden = 0x10,
        SignBiasAlt = 0x20,
    }

    mask Mpeg2SeqFlag {
        Progressive = 1,
    }

    enum Mpeg2PicCodingType {
        I = 1,
        P = 2,
        B = 3,
        D = 4,
    }

    enum Mpeg2Pic {
        TopField = 1,
        BottomField = 2,
        Frame = 3,
    }

    mask Mpeg2PicFlag {
        TopFieldFirst = 0x1,
        FramePredDct = 0x2,
        ConcealmentMv = 0x4,
        QScaleType = 0x8,
        IntraVlc = 0x10,
        AltScan = 0x20,
        RepeatFirst = 0x40,
        Progressive = 0x80,
    }

    mask Hdr10Mastering {
        PrimariesXLow = 5,
        PrimariesXHigh = 37000,
        PrimariesYLow = 5,
        PrimariesYHigh = 42000,
        WhitePointXLow = 5,
        WhitePointXHigh = 37000,
        WhitePointYLow = 5,
        WhitePointYHigh = 42000,
        MaxLumaLow = 50000,
        MaxLumaHigh = 100000000,
        MinLumaLow = 1,
        MinLumaHigh = 50000,
    }

    mask Vp9LoopFilterFlag {
        DeltaEnabled = 0x1,
        DeltaUpdate = 0x2,
    }

    mask Vp9SegmentationFlag {
        Enabled = 0x1,
        UpdateMap = 0x2,
        TemporalUpdate = 0x4,
        UpdateData = 0x8,
        AbsOrDeltaUpdate = 0x10,
    }

    enum Vp9SegmentationLevel {
        AltQ,
        AltL,
        RefFrame,
        Skip,
        Max,
    }

    mask Vp9FrameFlag {
        KeyFrame = 0x1,
        ShowFrame = 0x2,
        ErrorResilient = 0x4,
        IntraOnly = 0x8,
        AllowHighPrecMv = 0x10,
        RefreshFrameCtx = 0x20,
        ParallelDecMode = 0x40,
        XSubsampling = 0x80,
        YSubsampling = 0x100,
        ColorRangeFullSwing = 0x200,
    }

    mask Vp9SignBias {
        Last = 0x1,
        Golden = 0x2,
        Alt = 0x4,
    }

    enum Vp9ResetFrameCtx {
        None,
        Spec,
        All,
    }

    enum Vp9InterpFilter {
        Eighttap,
        EighttapSmooth,
        EighttapSharp,
        Bilinear,
        Switchable,
    }

    enum Vp9ReferenceMode {
        SingleReference,
        CompoundReference,
        Select,
    }

    enum Vp9TxMode {
        Only4x4,
        Allow8x8,
        Allow16x16,
        Allow32x32,
        Select,
    }

    mask MemoryFlag {
        NonCoherent = 1,
    }

    mask PixFmtFlag: u8 {
        PremulAlpha = 0x1,
        SetCsc = 0x2,
    }

    mask CaptureCapabilityFlag {
        TemperFrame = 0x1000,
    }

    mask CaptureModeFlag {
        HighQuality = 1,
    }

    enum DvFormat {
        Progressive,
        Interlaced,
    }

    /// Polarities
    ///
    /// If bit is not set, it is assumed to be negative polarity
    mask DvSyncPol {
        VsyncPos = 1,
        HsyncPos = 2,
    }

    /// Timings standards
    mask DvBtStd {
        Cea861 = 1,
        Dmt = 2,
        Cvt = 4,
        Gtf = 8,
        Sdi = 16,
    }

    mask DvFlag {
        ReducedBlanking = 1,
        CanReduceFps = 2,
        ReducedFps = 4,
        HalfLine = 8,
        IsCeVideo = 16,
        FirstFieldExtraLine = 32,
        HasPictureAspect = 64,
        HasCea861Vic = 128,
        HasHdmiVic = 256,
        CanDetectReducedFps = 512,
    }

    enum DvTimingsType {
        T656_1120,
    }

    mask DvBtCapabilityFlag {
        Interlaced = 1,
        Progressive = 2,
        ReducedBlanking = 4,
        Custom = 8,
    }

    enum InputType {
        Tuner = 1,
        Camera = 2,
        Touch = 3,
    }

    mask InputStatusFlag {
        NoPower = 0x1,
        NoSignal = 0x2,
        NoColor = 0x4,
        Hflip = 0x10,
        Vflip = 0x20,
        NoHLock = 0x100,
        ColorKill = 0x200,
        NoVLock = 0x400,
        NoStdLock = 0x800,
        NoSync = 0x10000,
        NoEqu = 0x20000,
        NoCarrier = 0x40000,
        Macrovision = 0x1000000,
        NoAccess = 0x2000000,
        Vtr = 0x4000000,
    }

    mask InputCapabilityFlag {
        DvTimings = 0x2,
        CustomTimings = 0x2,
        Std = 0x4,
        NativeSize = 0x8,
    }

    enum OutputType {
        Modulator = 1,
        Analog = 2,
        Analogvgaoverlay = 3,
    }

    mask OutputCapabilityFlag {
        DvTimings = 0x2,
        CustomTimings = 0x2,
        Std = 0x4,
        NativeSize = 0x8,
    }

    enum PowerLineFrequency {
        Disabled,
        F50Hz,
        F60Hz,
        Auto,
    }

    enum ColorFx {
        None,
        Bw,
        Sepia,
        Negative,
        Emboss,
        Sketch,
        SkyBlue,
        GrassGreen,
        SkinWhiten,
        Vivid,
        Aqua,
        ArtFreeze,
        Silhouette,
        Solarization,
        Antique,
        SetCbCr,
        SetRgb,
    }

    enum MpegStreamType {
        Mpeg2Ps,
        Mpeg2Ts,
        Mpeg1Ss,
        Mpeg2Dvd,
        Mpeg1Vcd,
        Mpeg1SVcd,
    }

    enum MpegStreamVbiFmt {
        None,
        IvTv,
    }

    enum MpegAudioSamplingFreq {
        F44100,
        F48000,
        F32000,
    }

    enum MpegAudioEncoding {
        Layer1,
        Layer2,
        Layer3,
        Aac,
        Ac3,
    }

    enum MpegAudioL1Bitrate {
        B32K,
        B64K,
        B96K,
        B128K,
        B160K,
        B192K,
        B224K,
        B256K,
        B288K,
        B320K,
        B352K,
        B384K,
        B416K,
        B448K,
    }

    enum MpegAudioL2Bitrate {
        B32K,
        B48K,
        B56K,
        B64K,
        B80K,
        B96K,
        B112K,
        B128K,
        B160K,
        B192K,
        B224K,
        B256K,
        B320K,
        B384K,
    }

    enum MpegAudioL3Bitrate {
        B32K,
        B40K,
        B48K,
        B56K,
        B64K,
        B80K,
        B96K,
        B112K,
        B128K,
        B160K,
        B192K,
        B224K,
        B256K,
        B320K,
    }

    enum MpegAudioMode {
        Stereo,
        JointStereo,
        Dual,
        Mono,
    }

    enum MpegAudioModeExtension {
        Bound4,
        Bound8,
        Bound12,
        Bound16,
    }

    enum MpegAudioEmphasis {
        None,
        E50Div15uS,
        CCITTJ17,
    }

    enum MpegAudioCrc {
        None,
        Crc16,
    }

    enum MpegAudioAc3Bitrate {
        B32K,
        B40K,
        B48K,
        B56K,
        B64K,
        B80K,
        B96K,
        B112K,
        B128K,
        B160K,
        B192K,
        B224K,
        B256K,
        B320K,
        B384K,
        B448K,
        B512K,
        B576K,
        B640K,
    }

    enum MpegAudioDecPlayback {
        Auto,
        Stereo,
        Left,
        Right,
        Mono,
        SwappedStereo,
    }

    enum MpegVideoEncoding {
        Mpeg1,
        Mpeg2,
        Mpeg4Avc,
    }

    enum MpegVideoAspect {
        A1x1,
        A3x4,
        A16x9,
        A221x100,
    }

    enum MpegVideoBitrateMode {
        Vbr,
        Cbr,
        Cq,
    }

    enum MpegVideoHeaderMode {
        Separate,
        JoinedWith1StFrame,
    }

    enum MpegVideoMultiSliceMode {
        Single,
        MaxMb,
        MaxBytes,
    }

    enum MpegVideoMpeg2Level {
        Low,
        Main,
        High1440,
        High,
    }

    enum MpegVideoMpeg2Profile {
        Simple,
        Main,
        SnrScalable,
        SpatiallyScalable,
        High,
        Multiview,
    }

    enum MpegVideoH264EntropyMode {
        CAVLC,
        CABAC,
    }

    enum MpegVideoH264Level {
        L10,
        L1B,
        L11,
        L12,
        L13,
        L20,
        L21,
        L22,
        L30,
        L31,
        L32,
        L40,
        L41,
        L42,
        L50,
        L51,
        L52,
        L60,
        L61,
        L62,
    }

    enum MpegVideoLoopFilterMode {
        Enabled,
        Disabled,
        DisabledAtSliceBoundary,
    }

    enum MpegVideoH264Profile {
        Baseline,
        ConstrainedBaseline,
        Main,
        Extended,
        High,
        High10,
        High422,
        High444Predictive,
        High10Intra,
        High422Intra,
        High444Intra,
        CAVLC444Intra,
        ScalableBaseline,
        ScalableHigh,
        ScalableHighIntra,
        StereoHigh,
        MultiviewHigh,
        ConstrainedHigh,
    }

    enum MpegVideoH264VuiSarIdc {
        Unspecified,
        I1x1,
        I12x11,
        I10x11,
        I16x11,
        I40x33,
        I24x11,
        I20x11,
        I32x11,
        I80x33,
        I18x11,
        I15x11,
        I64x33,
        I4x3,
        I3x2,
        I2x1,
        Extended,
    }

    enum MpegVideoH264SeiFpArrangementType {
        CheckerBoard,
        Column,
        Row,
        SideBySide,
        TopBottom,
        Temporal,
    }

    enum MpegVideoH264FmoMapType {
        InterleavedSlices,
        ScatteredSlices,
        ForegroundWithLeftOver,
        BoxOut,
        RasterScan,
        WipeScan,
        Explicit,
    }

    enum MpegVideoH264FmoChangeDir {
        Right,
        Left,
    }

    enum MpegVideoH264HierCodingType {
        B,
        P,
    }

    enum MpegVideoMpeg4Level {
        L0,
        L0B,
        L1,
        L2,
        L3,
        L3B,
        L4,
        L5,
    }

    enum MpegVideoMpeg4Profile {
        Simple,
        AdvancedSimple,
        Core,
        SimpleScalable,
        AdvancedCodingEfficiency,
    }

    enum Vp8NumPartitions {
        P1,
        P2,
        P4,
        P8,
    }

    enum Vp8NumRefFrames {
        F1,
        F2,
        F3,
    }

    enum Vp8GoldenFrameSel {
        UsePrev,
        UseRefPeriod,
    }

    enum MpegVideoVp8Profile {
        P0,
        P1,
        P2,
        P3,
    }

    enum MpegVideoVp9Profile {
        P0,
        P1,
        P2,
        P3,
    }

    enum MpegVideoVp9Level {
        L10,
        L11,
        L20,
        L21,
        L30,
        L31,
        L40,
        L41,
        L50,
        L51,
        L52,
        L60,
        L61,
        L62,
    }

    enum MpegVideoHevcHierCodingType {
        B,
        P,
    }

    enum MpegVideoHevcProfile {
        Main,
        MainStillPicture,
        Main10,
    }

    enum MpegVideoHevcLevel {
        L1,
        L2,
        L21,
        L3,
        L31,
        L4,
        L41,
        L5,
        L51,
        L52,
        L6,
        L61,
        L62,
    }

    enum MpegVideoHevcTier {
        Main,
        High,
    }

    enum CidMpegVideoHevcLoopFilterMode {
        Disabled,
        Enabled,
        DisabledAtSliceBoundary,
    }

    enum CidMpegVideoHevcRefreshType {
        None,
        Cra,
        Idr,
    }

    enum CidMpegVideoHevcSizeOfLengthField {
        Size0,
        Size1,
        Size2,
        Size4,
    }

    enum MpegVideoFrameSkipMode {
        Disabled,
        LevelLimit,
        BufLimit,
    }

    enum MpegCx2341xVideoLumaSpatialFilterType {
        Off,
        F1dHor,
        F1dVer,
        F2dHvSeparable,
        F2dSymNonSeparable,
    }

    enum MpegCx2341xVideoChromaSpatialFilterType {
        Off,
        F1dHor,
    }

    enum MpegCx2341xVideoTemporalFilterMode {
        Manual,
        Auto,
    }

    enum MpegCx2341xVideoMedianFilterType {
        Off,
        Hor,
        Vert,
        HorVert,
        Diag,
    }

    enum MpegMfc51VideoForceFrameSkipMode {
        Disabled,
        LevelLimit,
        BufLimit,
    }

    enum MpegMfc51VideoForceFrameType {
        Disabled,
        IFrame,
        NotCoded,
    }

    enum ExposureAutoType {
        Auto,
        Manual,
        ShutterPriority,
        AperturePriority,
    }

    enum AutoNPresentWhiteBalance {
        Manual,
        Auto,
        Incandencent,
        Fluorescent,
        FluorescentH,
        Horizon,
        Daylight,
        Flash,
        Cloudy,
        Shade,
    }

    enum IsoSensitivityAutoType {
        Manual,
        Auto,
    }

    enum ExposureMetering {
        Average,
        CenterWeighted,
        Spot,
        Matrix,
    }

    enum SceneMode {
        None,
        Backlight,
        BeachShow,
        CandleLight,
        DawnDusk,
        FallColors,
        Fireworks,
        Landscape,
        Night,
        PartyIndoor,
        Portrait,
        Sports,
        Sunset,
        Text,
    }

    enum AutoFocusRange {
        Auto,
        Normal,
        Macro,
        Infinity,
    }

    enum Preemphasis {
        Disabled,
        P50uS,
        P75uS,
    }

    enum FlashLedMode {
        None,
        Flash,
        Torch,
    }

    enum FlashStrobeSource {
        Software,
        External,
    }

    enum JpegChromaSubsampling {
        S444,
        S422,
        S420,
        S411,
        S410,
        Gray,
    }

    enum DvTxMode {
        DviD,
        Hdmi,
    }

    enum DvRgbRange {
        Auto,
        Limited,
        Full,
    }

    enum DvItContentType {
        Graphics,
        Photo,
        Cinema,
        Game,
        NoItc,
    }

    enum Deempasis {
        Disabled,
        D50uS,
        D75Us,
    }

    enum DetectMdMode {
        Disabled,
        Global,
        ThresholdGrid,
        RegionGrid,
    }

    enum StatelessH264DecodeMode {
        SliceBased,
        FrameBased,
    }

    enum StatelessH264StartCode {
        None,
        AnnexB,
    }

    enum Field {
        /// Driver can choose from none, top, bottom, interlaced
        ///	depending on whatever it thinks is approximate
        Any,
        /// This device has no fields
        None,
        /// Top field only
        Top,
        /// Bottom field only
        Bottom,
        /// Both fields interlaced
        Interlaced,
        /// Both fields sequential into one buffer, top-bottom order
        SequentialTb,
        /// Same as above + bottom-top order
        SequentialBt,
        /// Both fields alternating into separate buffers
        Alternate,
        /// Both fields interlaced, top field first and the top
        /// field is transmitted first
        InterlacedTb,
        /// Both fields interlaced, top field first and the bottom
        /// field is transmitted first
        InterlacedBt,
    }

    enum BufferType {
        VideoCapture = 0x1,
        VideoOutput,
        VideoOverlay,
        VbiCapture,
        VbiOutput,
        SlicedVbiCapture,
        SlicedVbiOutput,
        VideoOutputOverlay,
        VideoCaptureMplane,
        VideoOutputMplane,
        SdrCapture,
        SdrOutput,
        MetaCapture,
        MetaOutput,
    }

    enum TunerType {
        Radio = 0x1,
        AnalogTv,
        DigitalTv,
        Sdr,
        Rf,
    }

    enum Memory {
        Mmap = 0x1,
        UserPtr,
        Overlay,
        DmaBuf,
    }

    /// Colorspace enum
    ///
    /// See also <http://vektor.theorem.ca/graphics/ycbcr/>
    enum ColorSpace {
        /// Default colorspace, i.e. let the driver figure it out.
        /// Can only be used with video capture.
        Default,
        /// SMPTE 170M: used for broadcast NTSC/PAL SDTV
        Smpte170M,
        /// Obsolete pre-1998 SMPTE 240M HDTV standard, superseded by Rec 709
        Smpte240M,
        /// Rec.709: used for HDTV
        Rec709,
        /// Deprecated, do not use. No driver will ever return this. This was
        /// based on a misunderstanding of the bt878 datasheet.
        Bt878,
        /// NTSC 1953 colorspace. This only makes sense when dealing with
        /// really, really old NTSC recordings. Superseded by SMPTE 170M.
        C470SystemM,
        /// EBU Tech 3213 PAL/SECAM colorspace.
        C470SystemBg,
        /// Effectively shorthand for SRGB, YCBCR_ENC_601
        /// and QUANTIZATION_FULL_RANGE. To be used for (Motion-)JPEG.
        Jpeg,
        /// For RGB colorspaces such as produces by most webcams.
        Srgb,
        /// opRGB colorspace
        Oprgb,
        /// BT.2020 colorspace, used for UHDTV.
        Bt2020,
        /// Raw colorspace: for RAW unprocessed images
        Raw,
        /// DCI-P3 colorspace, used by cinema projectors
        DciP3,
    }

    /// Mapping of [XferFunc::Default] to actual transfer functions
    /// for the various colorspaces
    enum XferFunc: u8 {
        Default,
        F709,
        Srgb,
        Oprgb,
        Smpte240M,
        None,
        DciP3,
        Smpte2084,
    }

    /// Mapping of [YcbcrEncoding::Default] to actual encodings for the
    /// various colorspaces
    enum YcbcrEncoding: u8 {
        Default,
        E601,
        E709,
        Xv601,
        Xv709,
        Sycc,
        Bt2020,
        Bt2020ConstLum,
        Smpte240M,
    }

    enum HsvEncoding: u8 {
        E180 = 0x80,
        E256 = 0x81,
    }

    /// The default for R'G'B' quantization is always full range.
    /// For Y'CbCr the quantization is always limited range, except
    /// for [ColorSpace::Jpeg]: this is full range.
    enum Quantization: u8 {
        Default,
        FullRange,
        LimRange,
    }

    enum Priority {
        Unset,
        Background,
        Interactive,
        Record,
    }

    enum FwhtVersion {
        V3 = 0x3,
    }

    mask FwhtFlag {
        ComponentsNumOffset = 0x10,
        PixEncOffset = 0x13,
        PixEncYuv = 0x80000,
        PixEncRgb = 0x100000,
        PixEncHsv = 0x180000,
    }

    mask CapabilityFlag {
        VideoCapture = 0x1,
        VideoOutput = 0x2,
        VideoOverlay = 0x4,
        VbiCapture = 0x10,
        VbiOutput = 0x20,
        SlicedVbiCapture = 0x40,
        SlicedVbiOutput = 0x80,
        RdsCapture = 0x100,
        VideoOutputOverlay = 0x200,
        HwFreqSeek = 0x400,
        RdsOutput = 0x800,
        VideoCaptureMplane = 0x1000,
        VideoOutputMplane = 0x2000,
        VideoM2mMplane = 0x4000,
        VideoM2m = 0x8000,
        Tuner = 0x10000,
        Audio = 0x20000,
        Radio = 0x40000,
        Modulator = 0x80000,
        SdrCapture = 0x100000,
        ExtPixFormat = 0x200000,
        SdrOutput = 0x400000,
        MetaCapture = 0x800000,
        ReadWrite = 0x1000000,
        AsyncIo = 0x2000000,
        Streaming = 0x4000000,
        MetaOutput = 0x8000000,
        Touch = 0x10000000,
        IoMc = 0x20000000,
        DeviceCaps = 0x80000000,
    }

    mask BufferCapabilityFlag {
        SupportsMmap = 0x1,
        SupportsUserPtr = 0x2,
        SupportsDmaBuf = 0x4,
        SupportsRequests = 0x8,
        SupportsPrphanedBufs = 0x10,
        SupportsM2mHoldCaptureBuf = 0x20,
        SupportsMmapCacheHints = 0x40,
    }

    mask BufferFlag {
        Mapped = 0x1,
        Queued = 0x2,
        Done = 0x4,
        KeyFrame = 0x8,
        PFrame = 0x10,
        BFrame = 0x20,
        Error = 0x40,
        InRequest = 0x80,
        TimeCode = 0x100,
        M2mHoldCaptureBuf = 0x200,
        Prepared = 0x400,
        NoCacheInvalidate = 0x800,
        NoCacheClean = 0x1000,
        TimestampMask = 0xe000,
        //TimestampUnknown = 0x0,
        //TimestampMonotonic = 0x2000,
        //TimestampCopy = 0x4000,
        TimestampSrcMask = 0x70000,
        //TimestampSrcEof = 0x0,
        //TimestampSrcSoe = 0x10000,
        Last = 0x100000,
        RequestFd = 0x800000,
    }

    enum Timestamp {
        Unknown = 0x0,
        Monotonic = 0x2000,
        Copy = 0x4000,
    }

    enum TimestampSrc {
        Eof = 0x0,
        Soe = 0x10000,
    }

    mask FrameBufferCapabilityFlag {
        ExternOverlay = 0x1,
        ChromaKey = 0x2,
        ListClipping = 0x4,
        BitmapClipping = 0x8,
        LocalAlpha = 0x10,
        GlobalAlpha = 0x20,
        LocalInvAlpha = 0x40,
        SrcChromaKey = 0x80,
    }

    mask FrameBufferFlag {
        Primary = 0x1,
        Overlay = 0x2,
        ChromaKey = 0x4,
        LocalAlpha = 0x8,
        GlobalAlpha = 0x10,
        LocalInvAlpha = 0x20,
        SrcChromaKey = 0x40,
    }

    mask TunerCapabilityFlag {
        Low = 0x1,
        Norm = 0x2,
        HwSeekBounded = 0x4,
        HwSeekWrap = 0x8,
        Stereo = 0x10,
        Lang2 = 0x20,
        Sap = 0x20,
        Lang1 = 0x40,
        Rds = 0x80,
        RdsBlockIo = 0x100,
        RdsControls = 0x200,
        FreqBands = 0x400,
        HwSeekProgLim = 0x800,
        F1Hz = 0x1000,
    }

    mask TunerSubFlag {
        Mono = 0x1,
        Stereo = 0x2,
        Lang2 = 0x4,
        Sap = 0x4,
        Lang1 = 0x8,
        Rds = 0x10,
    }

    mask TunerModeFlag {
        Mono = 0x0,
        Stereo = 0x1,
        Lang2 = 0x2,
        Sap = 0x2,
        Lang1 = 0x3,
        Lang1Lang2 = 0x4,
    }

    mask RdsBlockFlag {
        Msk = 0x7,
        A = 0x0,
        B = 0x1,
        C = 0x2,
        D = 0x3,
        CAlt = 0x4,
        Invalid = 0x7,
        Corrected = 0x40,
        Error = 0x80,
    }

    mask FmtFlag {
        Compressed = 0x1,
        Emulated = 0x2,
        ContinuousByteStream = 0x4,
        DynResolution = 0x8,
        EncCapFrameInterval = 0x10,
        CscColorSpace = 0x20,
        CscXferFunc = 0x40,
        CscYcbcrEnc = 0x80,
        CscHsvEnc = 0x80,
        CscQuantization = 0x100,
    }

    enum FrmSizeType {
        Discrete = 0x1,
        Continuous,
        Stepwise,
    }

    enum FrmIvalType {
        Discrete = 0x1,
        Continuous,
        Stepwise,
    }

    enum TimeCodeType {
        T24fps = 0x1,
        T25fps,
        T30fps,
        T50fps,
        T60fps,
    }

    mask TimeCodeFlag {
        DropFrame = 1,
        ColorFrame = 2,
    }

    mask TimeCodeUserBits {
        Field = 0xc,
        UserDefined = 0x0,
        B8bitChars = 0x8,
    }

    mask JpegMarker {
        Dht = 0x8,
        Dqt = 0x10,
        Dri = 0x20,
        Com = 0x40,
        App = 0x80,
    }

    enum CtrlType {
        Integer = 0x1,
        Boolean,
        Menu,
        Button,
        Integer64,
        CtrlClass,
        String,
        BitMask,
        IntegerMenu,
        U8 = 0x100,
        U16 = 0x101,
        U32 = 0x102,
        Area = 0x106,
        Hdr10CllInfo = 0x110,
        Hdr10MasteringDisplay = 0x111,
        H264Sps = 0x200,
        H264Pps = 0x201,
        H264ScalingMatrix = 0x202,
        H264SliceParams = 0x203,
        H264DecodeParams = 0x204,
        H264PredWeights = 0x205,
        FwhtParams = 0x220,
        Vp8Params = 0x240,
        Mpeg2Quantisation = 0x250,
        Mpeg2Sequence = 0x251,
        Mpeg2Picture = 0x252,
        Vp9CompressedHdr = 0x260,
        Vp9Frame = 0x261,
    }

    enum CtrlClass {
        User = 0x980000,
        Codec = 0x990000,
        Camera = 0x9a0000,
        FmTx = 0x9b0000,
        Flash = 0x9c0000,
        Jpeg = 0x9d0000,
        ImageSource = 0x9e0000,
        ImageProc = 0x9f0000,
        Dv = 0xa00000,
        FmRx = 0xa10000,
        RfTuner = 0xa20000,
        Detect = 0xa30000,
        CodecStateless = 0xa40000,
        Colorimetry = 0xa50000,
    }

    mask CtrlFlag {
        Disabled = 0x1,
        Grabbed = 0x2,
        ReadOnly = 0x4,
        Update = 0x8,
        Inactive = 0x10,
        Slider = 0x20,
        WriteOnly = 0x40,
        Volatile = 0x80,
        HasPayload = 0x100,
        ExecuteOnWrite = 0x200,
        ModifyLayout = 0x400,
    }

    mask CtrlEnumFlag {
        NextCtrl = 0x80000000,
        NextCompound = 0x40000000,
    }

    mask BandModulation {
        Vsb = 0x2,
        Fm = 0x4,
        Am = 0x8,
    }

    mask AudioCapabilityFlag {
        Stereo = 0x1,
        Avl = 0x2,
    }

    enum AudioMode {
        Avl = 1,
    }

    enum EncIdxFrame {
        I,
        P,
        B,
        //Mask = 0xf,
    }

    enum EncCmd {
        Start,
        Stop,
        Pause,
        Resume,
    }

    mask EncCmdFlag {
        StopAtGopEnd = 0x1,
    }

    enum DecCmd {
        Start,
        Stop,
        Pause,
        Resume,
        Flush,
    }

    mask DecCmdFlag {
        StartMuteAudio = 0x1,
        PauseToBlack = 0x1,
        StopToBlack = 0x1,
        StopImmediately = 0x2,
    }

    enum DecStartFmt {
        None,
        Gop,
    }

    mask VbiFlags {
        Unsync = 0x1,
        Interlaced = 0x2,
    }

    mask SlicedVbiType {
        TeletextB = 0x0001,
        Vps = 0x0400,
        Caption525 = 0x1000,
        Wss625 = 0x4000,
        Vbi525 = Self::Caption525.bits,
        Vbi625 = Self::TeletextB.bits | Self::Vps.bits | Self::Wss625.bits,
    }

    enum MpegVbiIvtvType {
        TeletextB = 0x1,
        Caption525 = 0x4,
        Wss625 = 0x5,
        Vps = 0x7,
    }

    enum EventType {
        All,
        Vsync,
        Eos,
        Ctrl,
        FrameSync,
        SourceChange,
        MotionDet,
        PrivateStart = 0x08000000,
    }

    mask EventCtrlChangeFlag {
        Value = 0x1,
        Flags = 0x2,
        Range = 0x4,
    }

    mask EventSrcChangeFlag {
        Resolution = 0x1,
    }

    mask EventMotionDetFlag {
        HaveFrameSeq = 0x1,
    }

    mask EventSubFlag {
        SendInitial = 0x1,
        AllowFeedback = 0x2,
    }

    mask ChipMatch {
        Bridge = 0x0,
        Subdev = 0x4,

        /* The following four defines are no longer in use */
        Host = 0x0,
        I2cDriver = 0x1,
        I2cAddr = 0x2,
        Ac97 = 0x3,
    }

    mask ChipFlag {
        Readable = 0x1,
        Writable = 0x2,
    }
}

impl CtrlType {
    pub fn is_menu(&self) -> bool {
        matches!(self, CtrlType::Menu | CtrlType::IntegerMenu)
    }

    pub fn is_compound(&self) -> bool {
        *self as u32 >= 0x100
    }
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Interactive
    }
}

impl Field {
    pub fn has_top(&self) -> bool {
        matches!(
            self,
            Self::Top
                | Self::Interlaced
                | Self::InterlacedTb
                | Self::InterlacedBt
                | Self::SequentialTb
                | Self::SequentialBt
        )
    }

    pub fn has_bottom(&self) -> bool {
        matches!(
            self,
            Self::Bottom
                | Self::Interlaced
                | Self::InterlacedTb
                | Self::InterlacedBt
                | Self::SequentialTb
                | Self::SequentialBt
        )
    }

    pub fn has_both(&self) -> bool {
        matches!(
            self,
            Self::Interlaced
                | Self::InterlacedTb
                | Self::InterlacedBt
                | Self::SequentialTb
                | Self::SequentialBt
        )
    }

    pub fn has_t_or_b(&self) -> bool {
        matches!(self, Self::Bottom | Self::Top | Self::Alternate)
    }

    pub fn is_interlaced(&self) -> bool {
        matches!(
            self,
            Self::Interlaced | Self::InterlacedTb | Self::InterlacedBt
        )
    }

    pub fn is_sequential(&self) -> bool {
        matches!(self, Self::SequentialTb | Self::SequentialBt)
    }
}

impl BufferType {
    pub fn is_multiplanar(&self) -> bool {
        matches!(self, Self::VideoCaptureMplane | Self::VideoOutputMplane)
    }

    pub fn is_output(&self) -> bool {
        matches!(
            self,
            Self::VideoOutput
                | Self::VideoOutputMplane
                | Self::VideoOverlay
                | Self::VideoOutputOverlay
                | Self::VbiOutput
                | Self::SlicedVbiOutput
                | Self::SdrOutput
                | Self::MetaOutput
        )
    }

    pub fn is_capture(&self) -> bool {
        !self.is_output()
    }
}

impl BufferFlag {
    /// Get timestamp type
    pub fn timestamp(self) -> Timestamp {
        unsafe { core::mem::transmute(self.bits & Self::TimestampMask.bits) }
    }

    /// Get source timestamp type
    pub fn timestamp_src(self) -> TimestampSrc {
        unsafe { core::mem::transmute(self.bits & Self::TimestampSrcMask.bits) }
    }
}

impl ColorSpace {
    /// Determine how [ColorSpace::Default] should map to a proper colorspace.
    /// This depends on whether this is a SDTV image (use SMPTE 170M), an
    /// HDTV image (use Rec. 709), or something else (use sRGB).
    pub fn map_default(is_sdtv: bool, is_hdtv: bool) -> Self {
        if is_sdtv {
            Self::Smpte170M
        } else if is_hdtv {
            Self::Rec709
        } else {
            Self::Srgb
        }
    }
}

impl From<ColorSpace> for XferFunc {
    /// Determine how [XferFunc::Default] should map to a proper transfer function.
    /// This depends on the colorspace.
    fn from(colsp: ColorSpace) -> Self {
        match colsp {
            ColorSpace::Oprgb => Self::Oprgb,
            ColorSpace::Smpte240M => Self::Smpte240M,
            ColorSpace::DciP3 => Self::DciP3,
            ColorSpace::Raw => Self::None,
            ColorSpace::Srgb | ColorSpace::Jpeg => Self::Srgb,
            _ => Self::F709,
        }
    }
}

impl From<ColorSpace> for YcbcrEncoding {
    /// Determine how [YcbcrEncoding::Default] should map to a proper Y'CbCr encoding.
    /// This depends on the colorspace.
    fn from(colsp: ColorSpace) -> Self {
        match colsp {
            ColorSpace::Rec709 | ColorSpace::DciP3 => Self::E709,
            ColorSpace::Bt2020 => Self::Bt2020,
            ColorSpace::Smpte240M => Self::Smpte240M,
            _ => Self::E601,
        }
    }
}

impl Quantization {
    /// Determine how [Quantization::Default] should map to a proper quantization.
    /// This depends on whether the image is RGB or not, the colorspace.
    /// The Y'CbCr encoding is not used anymore, but is still there for backwards
    /// compatibility.
    pub fn map_default(is_rgb_or_hsv: bool, colsp: ColorSpace) -> Self {
        if is_rgb_or_hsv || matches!(colsp, ColorSpace::Jpeg) {
            Self::FullRange
        } else {
            Self::LimRange
        }
    }
}
