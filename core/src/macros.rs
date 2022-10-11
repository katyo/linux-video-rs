macro_rules! enum_impl {
    ($($(#[$($meta:meta)*])* $kind:ident $type:ident $(: $repr:ident)* {
        $($(#[$($var_meta:meta)*])*
          $var_name:ident $(= $var_data:expr)*,)*
    })*) => {
        $(
            enum_impl!(@$kind $(#[$($meta)*])* $type $(: $repr)* {
                $($(#[$($var_meta)*])* $var_name $(= $var_data)*,)*
            });
        )*
    };

    (@$kind:ident $(#[$($meta:meta)*])* $type:ident {
        $($(#[$($var_meta:meta)*])*
          $var_name:ident $(= $var_data:expr)*,)*
    }) => {
        enum_impl!(@$kind $(#[$($meta)*])* $type: u32 {
            $($(#[$($var_meta)*])* $var_name $(= $var_data)*,)*
        });
    };

    (@enum $(#[$($meta:meta)*])* $type:ident: $repr:ident {
        $($(#[$($var_meta:meta)*])*
          $var_name:ident $(= $var_data:expr)*,)*
    }) => {
        $(#[$($meta)*])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr($repr)]
        pub enum $type {
            $($(#[$($var_meta)*])* $var_name $(= $var_data)*,)*
        }

        impl core::convert::TryFrom<$repr> for $type {
            type Error = $repr;

            #[allow(non_upper_case_globals)]
            fn try_from(data: $repr) -> core::result::Result<Self, Self::Error> {
                $(pub const $var_name: $repr = $type::$var_name as _;)*

                Ok(match data {
                    $($var_name => Self::$var_name,)*
                    _ => return Err(data),
                })
            }
        }

        impl From<$type> for $repr {
            fn from(data: $type) -> Self {
                data as _
            }
        }

        impl core::ops::Deref for $type {
            type Target = $repr;

            fn deref(&self) -> &Self::Target {
                unsafe { &*(self as *const $type as *const $repr) }
            }
        }

        impl AsRef<$repr> for $type {
            fn as_ref(&self) -> &$repr {
                unsafe { &*(self as *const $type as *const $repr) }
            }
        }

        impl core::fmt::Display for $type {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self {
                    $(Self::$var_name => f.write_str(stringify!($var_name)),)*
                }
            }
        }
    };

    (@mask $(#[$($meta:meta)*])* $type:ident: $repr:ident {
        $($(#[$($var_meta:meta)*])*
          $var_name:ident = $var_data:expr,)*
    }) => {
        $(#[$($meta)*])*
        #[bitmask_enum::bitmask($repr)]
        pub enum $type {
            $($(#[$($var_meta)*])* $var_name = $var_data,)*
        }

        impl core::fmt::Display for $type {
            #[allow(unused_assignments)]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                let mut is_set = false;
                $(
                    if self.contains(Self::$var_name) {
                        if is_set {
                            f.write_str("|")?;
                        } else {
                            is_set = true;
                        }
                        f.write_str(stringify!($var_name))?;
                    }
                )*
                Ok(())
            }
        }
    };
}

macro_rules! trivial_impls {
    ($($type:ty {
        $($(#[$($entry_meta:meta)*])* $entry_func:ident $entry_name:ident $(($($entry_arg:tt)*))*: $entry_type:ty $({$($entry_opt:tt)*})*,)*
    })*) => {
        $(
            impl $type {
                $(
                    trivial_impls!(@$entry_func $(#[$($entry_meta)*])* $entry_name $(($($entry_arg)*))*: $entry_type $({$($entry_opt)*})*);
                )*
            }
        )*
    };

    /*
    (@get $(#[$($entry_meta:meta)*])* $entry_name:ident: $entry_type:ty) => {
        $(#[$($entry_meta)*])*
        pub fn $entry_name(&self) -> $entry_type {
            self.$entry_name
        }
    };

    (@get $(#[$($entry_meta:meta)*])* $entry_name:ident ($entry_field:ident): $entry_type:ty) => {
        $(#[$($entry_meta)*])*
        pub fn $entry_name(&self) -> $entry_type {
            self.$entry_field
        }
    };
    */

    (@getstr $(#[$($entry_meta:meta)*])* $entry_name:ident: $entry_type:ty) => {
        $(#[$($entry_meta)*])*
        pub fn $entry_name(&self) -> $entry_type {
            crate::utils::get_str_unchecked(&self.$entry_name)
        }
    };

    (@getstr $(#[$($entry_meta:meta)*])* $entry_name:ident ($entry_field:ident): $entry_type:ty) => {
        $(#[$($entry_meta)*])*
        pub fn $entry_name(&self) -> $entry_type {
            crate::utils::get_str_unchecked(&self.$entry_field)
        }
    };
}

macro_rules! unsafe_call {
    ($res:expr) => {
        unsafe { $res }.map_err(crate::Error::from)
    };
}
