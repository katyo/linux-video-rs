use crate::{calls, consts::*, ctrlid::*, enums::*, structs::*, utils, Internal, Result};
use core::mem::{ManuallyDrop, MaybeUninit};
use std::os::unix::io::RawFd;

impl VersionTriple {
    pub fn major(&self) -> u8 {
        self.major
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

trivial_impls! {
    Capability {
        /// Driver name
        getstr driver: &str,
        /// Card name
        getstr card: &str,
        /// Bus name
        getstr bus(bus_info): &str,
        /// Driver version
        get version: VersionTriple,
        /// Capability flags
        get capabilities: CapabilityFlag,
        /// Device capability flags
        get device_capabilities(device_caps): CapabilityFlag,
    }
}

impl Internal<Capability> {
    /// Query capabilities from file descriptor
    pub fn query(fd: RawFd) -> Result<Self> {
        let mut cap = MaybeUninit::zeroed();

        let cap = unsafe_call!(calls::query_cap(fd, cap.as_mut_ptr()).map(|_| cap.assume_init()))?;

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

impl Internal<QueryCtrl> {
    pub fn query(fd: RawFd, id: u32) -> Result<Self> {
        let mut ctrl = MaybeUninit::<QueryCtrl>::zeroed();

        let ctrl = unsafe_call!({
            ctrl.assume_init_mut().id = id;
            calls::query_ctrl(fd, ctrl.as_mut_ptr()).map(|_| ctrl.assume_init())
        })?;

        utils::check_str(&ctrl.name)?;

        Ok(ctrl.into())
    }

    pub fn query_next(fd: RawFd, prev_id: u32) -> Result<Option<Self>> {
        Self::query(
            fd,
            prev_id | (CtrlEnumFlag::NextCtrl | CtrlEnumFlag::NextCompound).bits(),
        )
        .map(Some)
        .or_else(|error| {
            if error.kind() == std::io::ErrorKind::InvalidInput {
                Ok(None)
            } else {
                Err(error)
            }
        })
    }
}

trivial_impls! {
    QueryCtrl {
        /// Control identifier
        get id: u32,
        /// Control type
        get type_: CtrlType,
        /// Control name
        getstr name: &str,
        /// Minimum value
        get min(minimum): i32,
        /// Maximum value
        get max(maximum): i32,
        /// Step value
        get step: i32,
        /// Default value
        get default(default_value): i32,
        /// Control flags
        get flags: CtrlFlag,
    }
}

impl QueryCtrl {
    pub fn is_menu(&self) -> bool {
        self.type_.is_menu()
    }

    pub fn has_payload(&self) -> bool {
        self.flags.contains(CtrlFlag::HasPayload)
    }
}

impl core::fmt::Display for QueryCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match CtrlId::try_from(self.id()) {
            Ok(id) => id.fmt(f),
            Err(id) => id.fmt(f),
        }?;
        f.write_str(": ")?;
        self.type_().fmt(f)?;
        f.write_str(" '")?;
        self.name().fmt(f)?;
        f.write_str("' ")?;
        if !self.flags().is_none() {
            self.flags().fmt(f)?;
            f.write_str(" ")?;
        }
        f.write_str("(")?;
        self.min().fmt(f)?;
        f.write_str("..=")?;
        self.max().fmt(f)?;
        f.write_str(" step:")?;
        self.step().fmt(f)?;
        f.write_str(" def:")?;
        self.default().fmt(f)?;
        f.write_str(")")?;
        Ok(())
    }
}

impl From<QueryCtrl> for QueryExtCtrl {
    fn from(ctrl: QueryCtrl) -> Self {
        Self {
            id: ctrl.id,
            type_: ctrl.type_,
            name: ctrl.name,
            minimum: ctrl.minimum as _,
            maximum: if matches!(ctrl.type_, CtrlType::BitMask) {
                ctrl.maximum as u32 as _
            } else {
                ctrl.maximum as _
            },
            default_value: if matches!(ctrl.type_, CtrlType::BitMask) {
                ctrl.default_value as u32 as _
            } else {
                ctrl.default_value as _
            },
            step: ctrl.step as _,
            flags: ctrl.flags
                | if matches!(ctrl.type_, CtrlType::String) {
                    CtrlFlag::HasPayload
                } else {
                    CtrlFlag::none()
                },
            elems: 1,
            nr_of_dims: 0,
            elem_size: match ctrl.type_ {
                CtrlType::Integer64 => core::mem::size_of::<i64>() as _,
                CtrlType::String => (ctrl.maximum + 1) as _,
                _ => core::mem::size_of::<i32>() as _,
            },
            dims: [0; CTRL_MAX_DIMS],
            reserved: [0; 32],
        }
    }
}

impl Internal<QueryExtCtrl> {
    pub fn query(fd: RawFd, id: u32) -> Result<Self> {
        let mut ctrl = MaybeUninit::<QueryExtCtrl>::zeroed();

        let ctrl = unsafe_call!({
            ctrl.assume_init_mut().id = id;
            calls::query_ext_ctrl(fd, ctrl.as_mut_ptr()).map(|_| ctrl.assume_init())
        })?;

        utils::check_str(&ctrl.name)?;

        Ok(ctrl.into())
    }

    pub fn query_next(fd: RawFd, prev_id: u32) -> Result<Option<Self>> {
        Self::query(
            fd,
            prev_id | (CtrlEnumFlag::NextCtrl | CtrlEnumFlag::NextCompound).bits(),
        )
        .map(Some)
        .or_else(|error| {
            if error.kind() == std::io::ErrorKind::InvalidInput {
                Ok(None)
            } else {
                Err(error)
            }
        })
    }

    pub fn query_fallback(fd: RawFd, id: u32) -> Result<Self> {
        Self::query(fd, id).or_else(|error| {
            if error.kind() == std::io::ErrorKind::BrokenPipe {
                Internal::<QueryCtrl>::query(fd, id).map(|fb_ctrl| fb_ctrl.map(From::from))
            } else {
                Err(error)
            }
        })
    }

    pub fn query_next_fallback(fd: RawFd, prev_id: u32) -> Result<Option<Self>> {
        Self::query_fallback(
            fd,
            prev_id | (CtrlEnumFlag::NextCtrl | CtrlEnumFlag::NextCompound).bits(),
        )
        .map(Some)
        .or_else(|error| {
            if error.kind() == std::io::ErrorKind::InvalidInput {
                Ok(None)
            } else {
                Err(error)
            }
        })
    }
}

trivial_impls! {
    QueryExtCtrl {
        /// Control identifier
        get id: u32,
        /// Control type
        get type_: CtrlType,
        /// Control name
        getstr name: &str,
        /// Minimum value, inclusive
        ///
        /// This field gives a lower bound for the control.
        get min(minimum): i64,
        /// Maximum value, inclusive
        ///
        /// This field gives an upper bound for the control
        get max(maximum): i64,
        /// This field gives a step size for the control
        get step: u64,
        /// Default value of control
        ///
        /// The default value of a [CtrlType::Integer], [CtrlType::Boolean], [CtrlType::BitMask], [CtrlType::Menu] or [CtrlType::IntegerMenu] control. Not valid for other types of controls
        get default(default_value): i64,
        /// Control flags
        get flags: CtrlFlag,
        /// The number of elements in the N-dimensional array
        get elems: u32,
        /// The size of the value in bytes
        get elem_size: u32,
    }
}

impl QueryExtCtrl {
    pub fn is_menu(&self) -> bool {
        self.type_.is_menu()
    }

    pub fn has_payload(&self) -> bool {
        self.flags.contains(CtrlFlag::HasPayload)
    }

    /// The size of each dimension
    pub fn dims(&self) -> &[u32] {
        &self.dims[..=self.nr_of_dims as usize]
    }

    /// Size of value in bytes
    pub fn size(&self) -> u32 {
        self.elem_size * self.elems
    }
}

impl core::fmt::Display for QueryExtCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match CtrlId::try_from(self.id()) {
            Ok(id) => id.fmt(f),
            Err(id) => id.fmt(f),
        }?;
        f.write_str(": ")?;
        self.type_().fmt(f)?;
        f.write_str(" '")?;
        self.name().fmt(f)?;
        f.write_str("' ")?;
        if !self.flags().is_none() {
            self.flags().fmt(f)?;
            f.write_str(" ")?;
        }
        f.write_str("(")?;
        self.min().fmt(f)?;
        f.write_str("..=")?;
        self.max().fmt(f)?;
        f.write_str(" step:")?;
        self.step().fmt(f)?;
        f.write_str(" def:")?;
        self.default().fmt(f)?;
        f.write_str(")")?;
        Ok(())
    }
}

impl QueryMenu {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn index(&self) -> u32 {
        self.index
    }
}

impl Internal<QueryMenu> {
    pub fn query(fd: RawFd, id: u32, index: u32) -> Result<Option<Self>> {
        let mut menu = unsafe {
            QueryMenu {
                id,
                index,
                ..core::mem::zeroed()
            }
        };

        unsafe_call!(calls::query_menu(fd, &mut menu as *mut _))
            .map(|_| Some(Internal(menu)))
            .or_else(|error| {
                if error.kind() == std::io::ErrorKind::InvalidInput {
                    Ok(None)
                } else {
                    Err(error)
                }
            })
    }

    pub fn into_item(self, ctrl_type: CtrlType, index: u32) -> MenuItem {
        MenuItem {
            menu_item: self,
            ctrl_type,
            index,
        }
    }
}

/// Helper type to represent menu item
pub struct MenuItem {
    ctrl_type: CtrlType,
    index: u32,
    menu_item: Internal<QueryMenu>,
}

impl Internal<MenuItem> {
    pub fn query(
        fd: RawFd,
        ctrl_type: CtrlType,
        ctrl_id: u32,
        index: u32,
    ) -> Result<Option<Internal<MenuItem>>> {
        Internal::<QueryMenu>::query(fd, ctrl_id, index)
            .map(|ok| ok.map(|item| item.into_item(ctrl_type, index).into()))
    }
}

trivial_impls! {
    MenuItem {
        /// Item index
        get index: u32,
        /// Control type
        get type_(ctrl_type): CtrlType,
    }
}

impl MenuItem {
    pub fn name(&self) -> Option<&str> {
        if matches!(self.ctrl_type, CtrlType::Menu) {
            utils::get_str(unsafe { &self.menu_item.union_.name }).ok()
        } else {
            None
        }
    }

    pub fn value(&self) -> Option<i64> {
        if matches!(self.ctrl_type, CtrlType::IntegerMenu) {
            Some(unsafe { self.menu_item.union_.value })
        } else {
            None
        }
    }
}

impl core::fmt::Display for MenuItem {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.index().fmt(f)?;
        f.write_str(": ")?;
        if let Some(name) = self.name() {
            f.write_str("'")?;
            name.fmt(f)?;
            f.write_str("'")?;
        } else if let Some(value) = self.value() {
            value.fmt(f)?;
        }
        Ok(())
    }
}

pub trait RawValue {
    const TYPES: &'static [CtrlType];
}

pub trait RefValue<T> {
    fn try_ref<'a>(data: &'a T, ctrl: &QueryExtCtrl) -> Option<&'a Self>;
}

pub trait MutValue<T> {
    fn try_mut<'a>(data: &'a mut T, ctrl: &QueryExtCtrl) -> Option<&'a mut Self>;
}

/// Control value
pub struct Value<C: AsRef<QueryExtCtrl>> {
    ctrl: C,
    data: Internal<ExtControl>,
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

impl<C: AsRef<QueryExtCtrl>> From<C> for Internal<Value<C>> {
    fn from(ctrl: C) -> Self {
        let data = Internal::<ExtControl>::new(ctrl.as_ref());

        Self(Value { ctrl, data })
    }
}

impl<C: AsRef<QueryExtCtrl>> Internal<Value<C>> {
    /// Get value from device
    pub fn get(&mut self, fd: RawFd) -> Result<()> {
        let ctrls = MaybeUninit::<ExtControls>::zeroed();

        unsafe_call!({
            let mut ctrls = ctrls.assume_init();

            ctrls.count = 1;
            ctrls.controls = self.data.as_mut() as *mut _;

            calls::g_ext_ctrls(fd, &mut ctrls as *mut _)
        })?;

        Ok(())
    }

    /// Set value to device
    pub fn set(&self, fd: RawFd) -> Result<()> {
        Internal::from(self.as_ref()).set(fd)
    }
}

impl<C: AsRef<QueryExtCtrl>> Internal<&Value<C>> {
    /// Set value to device
    pub fn set(&self, fd: RawFd) -> Result<()> {
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

impl<T: RawValue> RefValue<ExtControl> for T {
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

impl<T: RawValue> MutValue<ExtControl> for T {
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

impl<const N: usize, T: RawValue> RefValue<ExtControl> for [T; N] {
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

impl<const N: usize, T: RawValue> MutValue<ExtControl> for [T; N] {
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

impl<const N: usize, const M: usize, T: RawValue> RefValue<ExtControl> for [[T; N]; M] {
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

impl<const N: usize, const M: usize, T: RawValue> MutValue<ExtControl> for [[T; N]; M] {
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

impl<const N: usize, const M: usize, const L: usize, T: RawValue> RefValue<ExtControl>
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

impl<const N: usize, const M: usize, const L: usize, T: RawValue> MutValue<ExtControl>
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

impl<const N: usize, const M: usize, const L: usize, const O: usize, T: RawValue>
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

impl<const N: usize, const M: usize, const L: usize, const O: usize, T: RawValue>
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
    ctrl: *const C,
    data: Internal<ExtControls>,
}

impl<C: AsRef<QueryExtCtrl>> FromIterator<C> for Internal<Values<C>> {
    fn from_iter<T: IntoIterator<Item = C>>(iter: T) -> Self {
        let mut ctrls = Vec::new();
        let mut datas = Vec::new();

        for ctrl in iter {
            datas.push(Internal::<ExtControl>::new(ctrl.as_ref()));
            ctrls.push(ctrl);
        }

        let data = MaybeUninit::<ExtControls>::zeroed();

        let data = unsafe {
            let mut data = data.assume_init();

            data.count = datas.len() as _;
            data.controls = datas.as_mut_ptr() as _;

            let _ = ManuallyDrop::new(datas);

            data
        }
        .into();

        let ctrl = {
            let ptr = ctrls.as_ptr();
            let _ = ManuallyDrop::new(ctrls);
            ptr
        };

        Self(Values { ctrl, data })
    }
}

impl<C: AsRef<QueryExtCtrl>> Drop for Values<C> {
    fn drop(&mut self) {
        for index in 0..self.len() {
            let ctrl = unsafe { &*self.ctrl.add(index) };
            let data: &mut Internal<ExtControl> =
                unsafe { &mut *(self.data.controls.add(index) as *mut _) };
            if ctrl.as_ref().has_payload() {
                Internal::from(data).del();
            }
        }
    }
}

impl<C: AsRef<QueryExtCtrl>> Values<C> {
    /// Number of values
    pub fn len(&self) -> usize {
        self.data.count as _
    }

    /// Check for empty
    pub fn is_empty(&self) -> bool {
        self.data.count < 1
    }

    /// Values controls
    pub fn controls(&self) -> &[C] {
        unsafe { core::slice::from_raw_parts(self.ctrl, self.data.count as _) }
    }

    /// Values
    pub fn values(&self) -> &[ExtControl] {
        unsafe { core::slice::from_raw_parts(self.data.controls, self.data.count as _) }
    }

    /// Mutable values
    pub fn values_mut(&mut self) -> &mut [ExtControl] {
        unsafe { core::slice::from_raw_parts_mut(self.data.controls, self.data.count as _) }
    }
}

impl<C: AsRef<QueryExtCtrl>> Internal<Values<C>> {
    /// Get values from device
    pub fn get(&mut self, fd: RawFd) -> Result<()> {
        unsafe_call!(calls::g_ext_ctrls(fd, self.data.as_mut() as *mut _))?;
        Ok(())
    }

    /// Set values to device
    pub fn set(&mut self, fd: RawFd) -> Result<()> {
        Internal(self.as_mut()).set(fd)
    }
}

impl<C: AsRef<QueryExtCtrl>> Internal<&mut Values<C>> {
    /// Set values to device
    pub fn set(&mut self, fd: RawFd) -> Result<()> {
        unsafe_call!(calls::s_ext_ctrls(fd, self.data.as_mut() as *mut _))?;
        Ok(())
    }
}
