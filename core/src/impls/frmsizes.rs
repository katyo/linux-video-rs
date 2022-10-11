use crate::{calls, types::*, Internal, Result};
use core::mem::MaybeUninit;
use std::os::unix::io::RawFd;

impl FrmSizeEnum {
    /// Get reference to size description
    pub fn try_ref<T: IsFrmSizeData>(&self) -> Option<&T> {
        if T::TYPES.contains(&self.type_()) {
            Some(unsafe { &*(&self.union_ as *const _ as *const T) })
        } else {
            None
        }
    }

    /// Get iterator over all supported sizes
    pub fn sizes(&self) -> FrmSizeIter<'_> {
        FrmSizeIter {
            sizes: self,
            index: 0,
        }
    }
}

pub struct FrmSizeIter<'i> {
    sizes: &'i FrmSizeEnum,
    index: u32,
}

impl<'i> Iterator for FrmSizeIter<'i> {
    type Item = Area;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == u32::MAX {
            return None;
        }

        match self.sizes.type_ {
            FrmSizeType::Discrete => {
                self.index = u32::MAX;
                Some(*self.sizes.try_ref::<FrmSizeDiscrete>().unwrap())
            }
            FrmSizeType::Stepwise => {
                let size = self
                    .sizes
                    .try_ref::<FrmSizeStepwise>()
                    .unwrap()
                    .try_get(self.index);

                if size.is_some() {
                    self.index += 1;
                } else {
                    self.index = u32::MAX;
                }

                size
            }
            FrmSizeType::Continuous => None, // FIXME:
        }
    }
}

impl<'i> core::iter::FusedIterator for FrmSizeIter<'i> {}

impl FrmSizeStepwise {
    /// Try get discrete frame size by index
    pub fn try_get(&self, index: u32) -> Option<FrmSizeDiscrete> {
        let width = self.min_width + self.step_width * index;
        let height = self.min_height + self.step_height * index;

        if width <= self.max_width && height <= self.max_height {
            Some(FrmSizeDiscrete { width, height })
        } else {
            None
        }
    }
}

impl Internal<FrmSizeEnum> {
    pub fn query(fd: RawFd, index: u32, pixel_format: FourCc) -> Result<Option<Self>> {
        let frm_size = MaybeUninit::<FrmSizeEnum>::zeroed();

        unsafe_call!({
            let mut frm_size = frm_size.assume_init();
            frm_size.index = index;
            frm_size.pixel_format = pixel_format;
            calls::enum_frame_sizes(fd, &mut frm_size).map(|_| frm_size)
        })
        .map(|frm_size| Some(frm_size.into()))
        .or_else(|error| {
            if error.kind() == std::io::ErrorKind::InvalidInput {
                Ok(None)
            } else {
                Err(error)
            }
        })
    }
}

pub trait IsFrmSizeData {
    const TYPES: &'static [FrmSizeType];
}

macro_rules! frmsize_impl {
    ($($type:ty: $($buf_type:ident)*,)*) => {
        $(
            impl IsFrmSizeData for $type {
                const TYPES: &'static [FrmSizeType] = &[ $(FrmSizeType::$buf_type,)* ];
            }
        )*

        impl core::fmt::Display for FrmSizeEnum {
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
                        $(FrmSizeType::$buf_type)|* => self.try_ref::<$type>()
                            .ok_or_else(Default::default)?.fmt(f),
                    )*
                    _ => Ok(()),
                }
            }
        }
    };
}

frmsize_impl! {
    FrmSizeDiscrete: Discrete,
    FrmSizeStepwise: Stepwise,
}

impl core::fmt::Display for FrmSizeStepwise {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.min_width.fmt(f)?;
        'x'.fmt(f)?;
        self.min_height.fmt(f)?;

        "..=".fmt(f)?;

        self.max_width.fmt(f)?;
        'x'.fmt(f)?;
        self.max_height.fmt(f)?;

        '+'.fmt(f)?;

        self.step_width.fmt(f)?;
        'x'.fmt(f)?;
        self.step_height.fmt(f)
    }
}
