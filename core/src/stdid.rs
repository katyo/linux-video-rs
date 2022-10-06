#[bitmask_enum::bitmask(u64)]
pub enum StdId {
    /* one bit for each */
    PalB = 0x00000001,
    PalB1 = 0x00000002,
    PalG = 0x00000004,
    PalH = 0x00000008,
    PalI = 0x00000010,
    PalD = 0x00000020,
    PalD1 = 0x00000040,
    PalK = 0x00000080,

    PalM = 0x00000100,
    PalN = 0x00000200,
    PalNc = 0x00000400,
    Pal60 = 0x00000800,

    /// BTSC
    NtscM = 0x00001000,
    /// EIA-J
    NtscMJp = 0x00002000,
    Ntsc443 = 0x00004000,
    /// FM A2
    NtscMKr = 0x00008000,

    SecamB = 0x00010000,
    SecamD = 0x00020000,
    SecamG = 0x00040000,
    SecamH = 0x00080000,
    SecamK = 0x00100000,
    SecamK1 = 0x00200000,
    SecamL = 0x00400000,
    SecamLc = 0x00800000,

    /* ATSC/HDTV */
    Atsc8Vsb = 0x01000000,
    Atsc16Vsb = 0x02000000,

    /*
     * Some macros to merge video standards in order to make live easier for the
     * drivers and V4L2 applications
     */

    /*
     * "Common" NTSC/M - It should be noticed that V4L2_STD_NTSC_443 is
     * Missing here.
     */
    Ntsc = Self::NtscM.bits | Self::NtscMJp.bits | Self::NtscMKr.bits,

    /* Secam macros */
    SecamDk = Self::SecamD.bits | Self::SecamK.bits | Self::SecamK1.bits,

    /* All Secam Standards */
    Secam = Self::SecamB.bits
        | Self::SecamG.bits
        | Self::SecamH.bits
        | Self::SecamDk.bits
        | Self::SecamL.bits
        | Self::SecamLc.bits,

    /* PAL macros */
    PalBg = Self::PalB.bits | Self::PalB1.bits | Self::PalG.bits,
    PalDk = Self::PalD.bits | Self::PalD1.bits | Self::PalK.bits,

    /*
     * "Common" PAL - This macro is there to be compatible with the old
     * V4L1 concept of "PAL": /BGDKHI.
     * Several PAL standards are missing here: /M, /N and /Nc
     */
    Pal = Self::PalBg.bits | Self::PalDk.bits | Self::PalH.bits | Self::PalI.bits,

    /* Chroma "agnostic" standards */
    B = Self::PalB.bits | Self::PalB1.bits | Self::SecamB.bits,
    G = Self::PalG.bits | Self::SecamG.bits,
    H = Self::PalH.bits | Self::SecamH.bits,
    L = Self::SecamL.bits | Self::SecamLc.bits,
    Gh = Self::G.bits | Self::H.bits,
    Dk = Self::PalDk.bits | Self::SecamDk.bits,
    Bg = Self::B.bits | Self::G.bits,
    Mn = Self::PalM.bits | Self::PalN.bits | Self::PalNc.bits | Self::Ntsc.bits,

    /* Standards where MTS/BTSC stereo could be found */
    Mts = Self::NtscM.bits | Self::PalM.bits | Self::PalN.bits | Self::PalNc.bits,

    /* Standards for Countries with 60Hz Line frequency */
    S52560 = Self::PalM.bits | Self::Pal60.bits | Self::Ntsc.bits | Self::Ntsc443.bits,
    /* Standards for Countries with 50Hz Line frequency */
    S62550 = Self::Pal.bits | Self::PalN.bits | Self::PalNc.bits | Self::Secam.bits,

    Atsc = Self::Atsc8Vsb.bits | Self::Atsc16Vsb.bits,

    /* Macros with none and all analog standards */
    Unknown = 0,
    All = Self::S52560.bits | Self::S62550.bits,
}
