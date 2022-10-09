use crate::{calls, traits::*, types::*, Internal, Result};
use core::mem::{ManuallyDrop, MaybeUninit};
use std::os::unix::io::RawFd;

/// Control value
pub struct Value<C: AsRef<QueryExtCtrl>> {
    ctrl: C,
    data: Internal<ExtControl>,
}

impl<C: AsRef<QueryExtCtrl>> From<C> for Value<C> {
    fn from(ctrl: C) -> Self {
        let data = Internal::<ExtControl>::new(ctrl.as_ref());

        Self { ctrl, data }
    }
}

impl<C: AsRef<QueryExtCtrl>> core::ops::Deref for Value<C> {
    type Target = C;
    fn deref(&self) -> &Self::Target {
        &self.ctrl
    }
}

impl<C: AsRef<QueryExtCtrl>> Value<C> {
    pub fn control(&self) -> &C {
        &self.ctrl
    }

    pub fn try_ref<T: RefValue<ExtControl>>(&self) -> Option<&T> {
        T::try_ref(&self.data, self.ctrl.as_ref())
    }

    pub fn try_mut<T: MutValue<ExtControl>>(&mut self) -> Option<&mut T> {
        T::try_mut(&mut self.data, self.ctrl.as_ref())
    }
}

impl<C: AsRef<QueryExtCtrl>> Drop for Value<C> {
    fn drop(&mut self) {
        if self.ctrl.as_ref().has_payload() {
            self.data.del();
        }
    }
}

impl<C: AsRef<QueryExtCtrl>> GetValue for Value<C> {
    /// Get value from device
    fn get(&mut self, fd: RawFd) -> Result<()> {
        let ctrls = MaybeUninit::<ExtControls>::zeroed();

        unsafe_call!({
            let mut ctrls = ctrls.assume_init();

            ctrls.count = 1;
            ctrls.controls = self.data.as_mut() as *mut _;

            calls::g_ext_ctrls(fd, &mut ctrls as *mut _)
        })?;

        Ok(())
    }
}

impl<C: AsRef<QueryExtCtrl>> SetValue for Value<C> {
    /// Set value to device
    fn set(&self, fd: RawFd) -> Result<()> {
        let req = MaybeUninit::<ExtControls>::zeroed();

        unsafe_call!({
            let mut req = req.assume_init();

            req.count = 1;
            req.controls = self.data.as_ref() as *const _ as *mut _;

            calls::s_ext_ctrls(fd, &mut req as *mut _)
        })?;

        Ok(())
    }
}

impl Internal<ExtControl> {
    pub fn new(ctrl: &QueryExtCtrl) -> Self {
        let data = MaybeUninit::<ExtControl>::zeroed();
        let size = ctrl.size();

        let ptr = if ctrl.has_payload() {
            let mut data = Vec::<u8>::with_capacity(size as _);
            let ptr = data.as_mut_ptr();
            let _ = ManuallyDrop::new(data);
            ptr
        } else {
            core::ptr::null_mut()
        };

        let data = unsafe {
            let mut data = data.assume_init();

            data.union_.ptr = ptr as _;

            data.id = ctrl.id;
            data.size = size;

            data
        };

        Self(data)
    }

    pub fn del(&mut self) {
        unsafe {
            let _ = Vec::from_raw_parts(self.union_.ptr as *mut u8, 0, self.size as _);
            //self.union_.ptr = core::ptr::null_mut();
        }
    }
}

impl RefValue<ExtControl> for str {
    fn try_ref<'a>(data: &'a ExtControl, ctrl: &QueryExtCtrl) -> Option<&'a Self> {
        if matches!(ctrl.type_, CtrlType::String) && ctrl.has_payload() {
            unsafe { core::ffi::CStr::from_ptr(data.union_.string as _) }
                .to_str()
                .ok()
        } else {
            None
        }
    }
}

impl<T: PlainData> RefValue<ExtControl> for T {
    fn try_ref<'a>(data: &'a ExtControl, ctrl: &QueryExtCtrl) -> Option<&'a Self> {
        if T::TYPES.contains(&ctrl.type_) {
            if ctrl.has_payload() {
                if core::mem::size_of::<T>() as u32 <= data.size {
                    return Some(unsafe { &*(data.union_.ptr as *const _) });
                }
            } else if core::mem::size_of::<T>() <= core::mem::size_of::<ExtControlUnion>() {
                #[allow(unaligned_references)]
                return Some(unsafe { &*(&data.union_.value as *const _ as *const _) });
            }
        }
        None
    }
}

impl<T: PlainData> MutValue<ExtControl> for T {
    fn try_mut<'a>(data: &'a mut ExtControl, ctrl: &QueryExtCtrl) -> Option<&'a mut Self> {
        if T::TYPES.contains(&ctrl.type_) {
            if ctrl.has_payload() {
                if core::mem::size_of::<T>() as u32 <= data.size {
                    return Some(unsafe { &mut *(data.union_.ptr as *mut _) });
                }
            } else if core::mem::size_of::<T>() <= core::mem::size_of::<ExtControlUnion>() {
                #[allow(unaligned_references)]
                return Some(unsafe { &mut *(&mut data.union_.value as *mut _ as *mut _) });
            }
        }
        None
    }
}

impl<const N: usize, T: PlainData> RefValue<ExtControl> for [T; N] {
    fn try_ref<'a>(data: &'a ExtControl, ctrl: &QueryExtCtrl) -> Option<&'a Self> {
        if T::TYPES.contains(&ctrl.type_)
            && ctrl.has_payload()
            && core::mem::size_of::<Self>() as u32 <= data.size
        {
            Some(unsafe { &*(data.union_.ptr as *const _) })
        } else {
            None
        }
    }
}

impl<const N: usize, T: PlainData> MutValue<ExtControl> for [T; N] {
    fn try_mut<'a>(data: &'a mut ExtControl, ctrl: &QueryExtCtrl) -> Option<&'a mut Self> {
        if T::TYPES.contains(&ctrl.type_)
            && ctrl.has_payload()
            && core::mem::size_of::<Self>() as u32 <= data.size
        {
            Some(unsafe { &mut *(data.union_.ptr as *mut _) })
        } else {
            None
        }
    }
}

impl<const N: usize, const M: usize, T: PlainData> RefValue<ExtControl> for [[T; N]; M] {
    fn try_ref<'a>(data: &'a ExtControl, ctrl: &QueryExtCtrl) -> Option<&'a Self> {
        if T::TYPES.contains(&ctrl.type_)
            && ctrl.has_payload()
            && core::mem::size_of::<Self>() as u32 <= data.size
        {
            Some(unsafe { &*(data.union_.ptr as *const _) })
        } else {
            None
        }
    }
}

impl<const N: usize, const M: usize, T: PlainData> MutValue<ExtControl> for [[T; N]; M] {
    fn try_mut<'a>(data: &'a mut ExtControl, ctrl: &QueryExtCtrl) -> Option<&'a mut Self> {
        if T::TYPES.contains(&ctrl.type_)
            && ctrl.has_payload()
            && core::mem::size_of::<Self>() as u32 <= data.size
        {
            Some(unsafe { &mut *(data.union_.ptr as *mut _) })
        } else {
            None
        }
    }
}

impl<const N: usize, const M: usize, const L: usize, T: PlainData> RefValue<ExtControl>
    for [[[T; N]; M]; L]
{
    fn try_ref<'a>(data: &'a ExtControl, ctrl: &QueryExtCtrl) -> Option<&'a Self> {
        if T::TYPES.contains(&ctrl.type_)
            && ctrl.has_payload()
            && core::mem::size_of::<Self>() as u32 <= data.size
        {
            Some(unsafe { &*(data.union_.ptr as *const _) })
        } else {
            None
        }
    }
}

impl<const N: usize, const M: usize, const L: usize, T: PlainData> MutValue<ExtControl>
    for [[[T; N]; M]; L]
{
    fn try_mut<'a>(data: &'a mut ExtControl, ctrl: &QueryExtCtrl) -> Option<&'a mut Self> {
        if T::TYPES.contains(&ctrl.type_)
            && ctrl.has_payload()
            && core::mem::size_of::<Self>() as u32 <= data.size
        {
            Some(unsafe { &mut *(data.union_.ptr as *mut _) })
        } else {
            None
        }
    }
}

impl<const N: usize, const M: usize, const L: usize, const O: usize, T: PlainData>
    RefValue<ExtControl> for [[[[T; N]; M]; L]; O]
{
    fn try_ref<'a>(data: &'a ExtControl, ctrl: &QueryExtCtrl) -> Option<&'a Self> {
        if T::TYPES.contains(&ctrl.type_)
            && ctrl.has_payload()
            && core::mem::size_of::<Self>() as u32 <= data.size
        {
            Some(unsafe { &*(data.union_.ptr as *const _) })
        } else {
            None
        }
    }
}

impl<const N: usize, const M: usize, const L: usize, const O: usize, T: PlainData>
    MutValue<ExtControl> for [[[[T; N]; M]; L]; O]
{
    fn try_mut<'a>(data: &'a mut ExtControl, ctrl: &QueryExtCtrl) -> Option<&'a mut Self> {
        if T::TYPES.contains(&ctrl.type_)
            && ctrl.has_payload()
            && core::mem::size_of::<Self>() as u32 <= data.size
        {
            Some(unsafe { &mut *(data.union_.ptr as *mut _) })
        } else {
            None
        }
    }
}

/// Control values
pub struct Values<C: AsRef<QueryExtCtrl>> {
    ctrls: Vec<C>,
    datas: Vec<Internal<ExtControl>>,
}

impl<C: AsRef<QueryExtCtrl>> FromIterator<C> for Values<C> {
    fn from_iter<T: IntoIterator<Item = C>>(iter: T) -> Self {
        let mut ctrls = Vec::new();
        let mut datas = Vec::new();

        for ctrl in iter {
            datas.push(Internal::<ExtControl>::new(ctrl.as_ref()));
            ctrls.push(ctrl);
        }

        Self { ctrls, datas }
    }
}

impl<C: AsRef<QueryExtCtrl>> Drop for Values<C> {
    fn drop(&mut self) {
        for index in 0..self.len() {
            let ctrl = &self.ctrls[index];
            let data = &mut self.datas[index];
            if ctrl.as_ref().has_payload() {
                Internal::from(data).del();
            }
        }
    }
}

impl<C: AsRef<QueryExtCtrl>> Values<C> {
    /// Number of values
    pub fn len(&self) -> usize {
        self.ctrls.len()
    }

    /// Check for empty
    pub fn is_empty(&self) -> bool {
        self.ctrls.is_empty()
    }

    /// Values controls
    pub fn controls(&self) -> &[C] {
        &self.ctrls
    }

    /// Get reference to value by index
    pub fn try_ref<T: RefValue<ExtControl>>(&self, index: usize) -> Option<&T> {
        if index < self.ctrls.len() {
            T::try_ref(&self.datas[index], self.ctrls[index].as_ref())
        } else {
            None
        }
    }

    /// Get mutable reference to value by index
    pub fn try_mut<T: MutValue<ExtControl>>(&mut self, index: usize) -> Option<&mut T> {
        if index < self.ctrls.len() {
            T::try_mut(&mut self.datas[index], self.ctrls[index].as_ref())
        } else {
            None
        }
    }
}

impl<C: AsRef<QueryExtCtrl>> GetValue for Values<C> {
    /// Get values from device
    fn get(&mut self, fd: RawFd) -> Result<()> {
        let mut ctrls = MaybeUninit::<ExtControls>::zeroed();

        unsafe {
            let mut ctrls = ctrls.assume_init();

            ctrls.count = self.ctrls.len() as _;
            ctrls.controls = self.datas.as_mut_ptr() as _;
        }

        unsafe_call!(calls::g_ext_ctrls(fd, ctrls.as_mut_ptr()))?;
        Ok(())
    }
}

impl<C: AsRef<QueryExtCtrl>> SetValue for Values<C> {
    /// Set values to device
    fn set(&self, fd: RawFd) -> Result<()> {
        let mut ctrls = MaybeUninit::<ExtControls>::zeroed();

        unsafe {
            let mut ctrls = ctrls.assume_init();

            ctrls.count = self.ctrls.len() as _;
            ctrls.controls = self.datas.as_ptr() as _;
        }

        unsafe_call!(calls::s_ext_ctrls(fd, ctrls.as_mut_ptr()))?;
        Ok(())
    }
}
