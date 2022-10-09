use crate::{calls, types::*, Internal, Result};
use core::mem::MaybeUninit;
use std::os::unix::io::RawFd;

impl FrmIvalEnum {
    /// Get reference to value
    pub fn try_ref<T: IsFrmIvalData>(&self) -> Option<&T> {
        if T::TYPES.contains(&self.type_()) {
            Some(unsafe { &*(&self.union_ as *const _ as *const T) })
        } else {
            None
        }
    }
}

impl Internal<FrmIvalEnum> {
    pub fn query(
        fd: RawFd,
        index: u32,
        pixel_format: FourCc,
        width: u32,
        height: u32,
    ) -> Result<Option<Self>> {
        let mut q = MaybeUninit::<FrmIvalEnum>::zeroed();

        unsafe_call!({
            {
                let q = q.assume_init_mut();

                q.index = index;
                q.pixel_format = pixel_format;
                q.width = width;
                q.height = height;
            }
            calls::enum_frame_intervals(fd, q.as_mut_ptr()).map(|_| q.assume_init())
        })
        .map(|q| Some(q.into()))
        .or_else(|error| {
            if error.kind() == std::io::ErrorKind::InvalidInput {
                Ok(None)
            } else {
                Err(error)
            }
        })
    }
}

pub trait IsFrmIvalData {
    const TYPES: &'static [FrmIvalType];
}

macro_rules! frmsize_impl {
    ($($type:ty: $($buf_type:ident)*,)*) => {
        $(
            impl IsFrmIvalData for $type {
                const TYPES: &'static [FrmIvalType] = &[ $(FrmIvalType::$buf_type,)* ];
            }
        )*

        impl core::fmt::Display for FrmIvalEnum {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                '#'.fmt(f)?;
                self.index.fmt(f)?;
                ' '.fmt(f)?;
                self.pixel_format.fmt(f)?;
                ' '.fmt(f)?;
                self.type_.fmt(f)?;
                ' '.fmt(f)?;
                match self.type_ {
                    $(
                        $(FrmIvalType::$buf_type)|* => self.try_ref::<$type>()
                            .ok_or_else(Default::default)?.fmt(f),
                    )*
                    _ => Ok(()),
                }
            }
        }
    };
}

frmsize_impl! {
    Fract: Discrete,
    FrmIvalStepwise: Stepwise,
}

impl core::fmt::Display for FrmIvalStepwise {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.min.fmt(f)?;
        "..=".fmt(f)?;
        self.max.fmt(f)?;
        '+'.fmt(f)?;
        self.step.fmt(f)
    }
}
