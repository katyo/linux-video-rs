use crate::FourCc;

impl core::convert::TryFrom<drm_fourcc::DrmFourcc> for FourCc {
    type Error = drm_fourcc::DrmFourcc;

    fn try_from(fourcc: drm_fourcc::DrmFourcc) -> Result<Self, Self::Error> {
        (fourcc as u32).try_into().map_err(|_| fourcc)
    }
}

impl core::convert::TryFrom<FourCc> for drm_fourcc::DrmFourcc {
    type Error = FourCc;

    fn try_from(fourcc: FourCc) -> Result<Self, Self::Error> {
        (fourcc as u32).try_into().map_err(|_| fourcc)
    }
}
