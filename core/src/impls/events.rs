use crate::{calls, types::*, Internal, IsTimestamp, Result};
use core::mem::MaybeUninit;
use std::os::unix::io::RawFd;

pub trait IsEventData {
    const TYPE: EventType;
}

macro_rules! event_data_impl {
    ($($type:ident: $event_type:ident,)*) => {
        $(
            impl IsEventData for $type {
                const TYPE: EventType = EventType::$event_type;
            }
        )*
    }
}

event_data_impl! {
    EventVsync: Vsync,
    EventCtrl: Ctrl,
    EventFrameSync: FrameSync,
    EventSrcChange: SourceChange,
    EventMotionDet: MotionDet,
}

impl Event {
    /// Try get reference to data of specific type
    pub fn data<T: IsEventData>(&self) -> Option<&T> {
        if self.type_ == T::TYPE {
            Some(unsafe { &*(&self.u as *const _ as *const T) })
        } else {
            None
        }
    }

    /// Get timestamp
    pub fn timestamp<T: IsTimestamp>(&self) -> T {
        T::from_time_spec(self.timestamp)
    }
}

impl Internal<Event> {
    /// Dequeue event
    pub fn dequeue(fd: RawFd) -> Result<Self> {
        let event = MaybeUninit::<Event>::uninit();

        unsafe_call!({
            let mut event = event.assume_init();

            calls::dq_event(fd, &mut event).map(|_| event.into())
        })
    }
}
