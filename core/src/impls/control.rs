use crate::{calls, types::*, utils, Internal, Result};
use core::mem::MaybeUninit;
use getset::CopyGetters;
use std::os::unix::io::RawFd;

impl Internal<QueryCtrl> {
    pub fn query(fd: RawFd, id: u32) -> Result<Self> {
        let ctrl = MaybeUninit::<QueryCtrl>::zeroed();

        let ctrl = unsafe_call!({
            let mut ctrl = ctrl.assume_init();
            ctrl.id = id;
            calls::query_ctrl(fd, &mut ctrl).map(|_| ctrl)
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
        /// Control name
        getstr name: &str,
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
            min: ctrl.min as _,
            max: if matches!(ctrl.type_, CtrlType::BitMask) {
                ctrl.max as u32 as _
            } else {
                ctrl.max as _
            },
            default: if matches!(ctrl.type_, CtrlType::BitMask) {
                ctrl.default as u32 as _
            } else {
                ctrl.default as _
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
                CtrlType::String => (ctrl.max + 1) as _,
                _ => core::mem::size_of::<i32>() as _,
            },
            dims: [0; CTRL_MAX_DIMS],
            reserved: [0; 32],
        }
    }
}

impl Internal<QueryExtCtrl> {
    pub fn query(fd: RawFd, id: u32) -> Result<Self> {
        let ctrl = MaybeUninit::<QueryExtCtrl>::zeroed();

        let ctrl = unsafe_call!({
            let mut ctrl = ctrl.assume_init();
            ctrl.id = id;
            calls::query_ext_ctrl(fd, &mut ctrl).map(|_| ctrl)
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
        /// Control name
        getstr name: &str,
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

    pub fn into_item(self, type_: CtrlType, index: u32) -> MenuItem {
        MenuItem {
            item: self,
            type_,
            index,
        }
    }
}

/// Helper type to represent menu item
#[derive(Clone, Copy, CopyGetters)]
pub struct MenuItem {
    /// Control type
    #[getset(get_copy = "pub")]
    type_: CtrlType,

    /// Item index
    #[getset(get_copy = "pub")]
    index: u32,

    item: Internal<QueryMenu>,
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

impl MenuItem {
    pub fn name(&self) -> Option<&str> {
        if matches!(self.type_, CtrlType::Menu) {
            utils::get_str(unsafe { &self.item.union_.name }).ok()
        } else {
            None
        }
    }

    pub fn value(&self) -> Option<i64> {
        if matches!(self.type_, CtrlType::IntegerMenu) {
            Some(unsafe { self.item.union_.value })
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
