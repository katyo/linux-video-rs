use crate::types::{TimeSpec, TimeVal};
use core::time::Duration;
use nix::sys::time::TimeValLike;
use std::time::SystemTime;

/// Something which can be used as timestamp
pub trait IsTimestamp {
    /// Convert from time val
    fn from_time_val(time_val: TimeVal) -> Self;

    /// Convert from time spec
    fn from_time_spec(time_spec: TimeSpec) -> Self;

    /// Convert into time val
    fn into_time_val(self) -> TimeVal;

    /// Convert into time spec
    fn into_time_spec(self) -> TimeSpec;
}

impl IsTimestamp for TimeVal {
    fn from_time_val(time_val: TimeVal) -> Self {
        time_val
    }

    fn from_time_spec(time_spec: TimeSpec) -> Self {
        Self::nanoseconds(time_spec.num_nanoseconds())
    }

    fn into_time_val(self) -> TimeVal {
        self
    }

    fn into_time_spec(self) -> TimeSpec {
        TimeSpec::nanoseconds(self.num_nanoseconds())
    }
}

impl IsTimestamp for Duration {
    fn from_time_val(time_val: TimeVal) -> Self {
        Duration::from_micros(time_val.num_microseconds() as _)
    }

    fn from_time_spec(time_spec: TimeSpec) -> Self {
        Duration::from_nanos(time_spec.num_nanoseconds() as _)
    }

    fn into_time_val(self) -> TimeVal {
        TimeVal::microseconds(self.as_micros() as _)
    }

    fn into_time_spec(self) -> TimeSpec {
        TimeSpec::from_duration(self)
    }
}

impl IsTimestamp for SystemTime {
    fn from_time_val(time_val: TimeVal) -> Self {
        SystemTime::UNIX_EPOCH + Duration::from_time_val(time_val)
    }

    fn from_time_spec(time_spec: TimeSpec) -> Self {
        SystemTime::UNIX_EPOCH + Duration::from_time_spec(time_spec)
    }

    fn into_time_val(self) -> TimeVal {
        self.duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .into_time_val()
    }

    fn into_time_spec(self) -> TimeSpec {
        self.duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .into_time_spec()
    }
}
