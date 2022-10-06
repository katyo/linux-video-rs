use crate::{calls, consts::*, interface::*, structs::*, utils, Internal, Result};
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

/// Control value
pub struct Value {
    type_: CtrlType,
    ctrl: Internal<ExtControl>,
}

impl From<&Internal<QueryExtCtrl>> for Internal<Value> {
    fn from(ctrl: &Internal<QueryExtCtrl>) -> Self {
        Self::new(ctrl.id, ctrl.type_, ctrl.size())
    }
}

impl Internal<Value> {
    pub fn new(id: u32, type_: CtrlType, size: u32) -> Self {
        let ctrl = Internal::<ExtControl>::new(id, type_, size);

        Self(Value { type_, ctrl })
    }

    /// Get value from device
    pub fn get(&mut self, fd: RawFd) -> Result<()> {
        let ctrls = MaybeUninit::<ExtControls>::zeroed();

        unsafe_call!({
            let mut ctrls = ctrls.assume_init();

            ctrls.count = 1;
            ctrls.controls = self.ctrl.as_mut() as *mut _;

            calls::g_ext_ctrls(fd, &mut ctrls as *mut _)
        })?;

        Ok(())
    }

    /// Set value to device
    pub fn set(&self, fd: RawFd) -> Result<()> {
        Internal::from(self.as_ref()).set(fd)
    }
}

impl Internal<&Value> {
    /// Set value to device
    pub fn set(&self, fd: RawFd) -> Result<()> {
        let ctrls = MaybeUninit::<ExtControls>::zeroed();

        unsafe_call!({
            let mut ctrls = ctrls.assume_init();

            ctrls.count = 1;
            ctrls.controls = self.ctrl.as_ref() as *const _ as *mut _;

            calls::s_ext_ctrls(fd, &mut ctrls as *mut _)
        })?;

        Ok(())
    }
}

impl Internal<ExtControl> {
    pub fn new(id: u32, type_: CtrlType, size: u32) -> Self {
        let ctrl = MaybeUninit::<ExtControl>::zeroed();

        let ptr = if type_.is_compound() {
            let mut data = Vec::<u8>::with_capacity(size as _);
            let ptr = data.as_mut_ptr();
            let _ = ManuallyDrop::new(data);
            ptr
        } else {
            core::ptr::null_mut()
        };

        let ctrl = unsafe {
            let mut ctrl = ctrl.assume_init();

            ctrl.union_.ptr = ptr as _;

            ctrl.id = id;
            ctrl.size = size;

            ctrl
        };

        Self(ctrl)
    }

    pub fn del(&mut self) {
        unsafe {
            let _ = Vec::from_raw_parts(self.union_.ptr as *mut u8, 0, self.size as _);
            //self.union_.ptr = core::ptr::null_mut();
        }
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        if self.type_.is_compound() {
            self.ctrl.del();
        }
    }
}

/// Control values
pub struct Values {
    ctrls: Internal<ExtControls>,
    types: *const CtrlType,
}

impl<'i> FromIterator<&'i QueryExtCtrl> for Internal<Values> {
    fn from_iter<T: IntoIterator<Item = &'i QueryExtCtrl>>(iter: T) -> Self {
        let mut ctrls = Vec::new();
        let mut types = Vec::new();

        for ctrl in iter {
            ctrls.push(Internal::<ExtControl>::new(
                ctrl.id,
                ctrl.type_,
                ctrl.size(),
            ));
            types.push(ctrl.type_);
        }

        let xctrls = MaybeUninit::<ExtControls>::zeroed();

        let ctrls = unsafe {
            let mut xctrls = xctrls.assume_init();

            xctrls.count = ctrls.len() as _;
            xctrls.controls = ctrls.as_mut_ptr() as _;

            let _ = ManuallyDrop::new(ctrls);

            xctrls
        }
        .into();

        let types = {
            let ptr = types.as_ptr();
            let _ = ManuallyDrop::new(types);
            ptr
        };

        Self(Values { ctrls, types })
    }
}

impl Drop for Values {
    fn drop(&mut self) {
        for index in 0..self.len() {
            let type_ = unsafe { *self.types.add(index) };
            let ctrl: &mut Internal<ExtControl> =
                unsafe { &mut *(self.ctrls.controls.add(index) as *mut _) };
            if type_.is_compound() {
                Internal::from(ctrl).del();
            }
        }
    }
}

impl Values {
    /// Number of values
    pub fn len(&self) -> usize {
        self.ctrls.count as _
    }

    /// Check for empty
    pub fn is_empty(&self) -> bool {
        self.ctrls.count < 1
    }

    /// Value types
    pub fn types(&self) -> &[CtrlType] {
        unsafe { core::slice::from_raw_parts(self.types, self.ctrls.count as _) }
    }

    /// Values
    pub fn values(&self) -> &[ExtControl] {
        unsafe { core::slice::from_raw_parts(self.ctrls.controls, self.ctrls.count as _) }
    }

    /// Mutable values
    pub fn values_mut(&mut self) -> &mut [ExtControl] {
        unsafe { core::slice::from_raw_parts_mut(self.ctrls.controls, self.ctrls.count as _) }
    }
}

impl Internal<Values> {
    /// Get values from device
    pub fn get(&mut self, fd: RawFd) -> Result<()> {
        unsafe_call!(calls::g_ext_ctrls(fd, self.ctrls.as_mut() as *mut _))?;
        Ok(())
    }

    /// Set values to device
    pub fn set(&mut self, fd: RawFd) -> Result<()> {
        Internal(self.as_mut()).set(fd)
    }
}

impl Internal<&mut Values> {
    /// Set values to device
    pub fn set(&mut self, fd: RawFd) -> Result<()> {
        unsafe_call!(calls::s_ext_ctrls(fd, self.ctrls.as_mut() as *mut _))?;
        Ok(())
    }
}
