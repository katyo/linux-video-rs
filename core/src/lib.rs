#![doc = include_str!("../README.md")]

#[cfg(not(target_os = "linux"))]
compile_error!("This crate support Linux only");

#[macro_use]
mod macros;

mod calls;
mod impls;
mod traits;
mod types;
mod utils;

pub mod private;

pub use impls::*;
pub use traits::*;
pub use types::*;

use private::*;

pub use std::io::{Error, Result};
