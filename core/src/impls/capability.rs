use crate::{calls, types::*, utils, Internal, Result};
use core::mem::MaybeUninit;
use std::os::unix::io::RawFd;

trivial_impls! {
    Capability {
        /// Driver name
        getstr driver: &str,
        /// Card name
        getstr card: &str,
        /// Bus name
        getstr bus(bus_info): &str,
    }
}

impl core::fmt::Display for VersionTriple {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.major.fmt(f)?;
        f.write_str(".")?;
        self.minor.fmt(f)?;
        f.write_str(".")?;
        self.patch.fmt(f)
    }
}

impl Internal<Capability> {
    /// Query capabilities from file descriptor
    pub fn query(fd: RawFd) -> Result<Self> {
        let cap = MaybeUninit::zeroed();

        let cap = unsafe_call!({
            let mut cap = cap.assume_init();
            calls::query_cap(fd, &mut cap).map(|_| cap)
        })?;

        utils::check_str(&cap.driver)?;
        utils::check_str(&cap.card)?;
        utils::check_str(&cap.bus_info)?;

        Ok(cap.into())
    }
}

impl core::fmt::Display for Capability {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str("driver: '")?;
        f.write_str(self.driver())?;
        f.write_str("', card: '")?;
        f.write_str(self.card())?;
        f.write_str("', bus: '")?;
        f.write_str(self.bus())?;
        f.write_str("', version: ")?;
        self.version().fmt(f)?;
        f.write_str(", capabilities: ")?;
        self.capabilities().fmt(f)?;
        f.write_str(", device capabilities: ")?;
        self.device_capabilities().fmt(f)
    }
}
