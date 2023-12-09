#![no_std]

pub mod sipo;
pub mod error;

pub use crate::sipo::{SipoShiftReg, SipoShiftRegPin};
pub use crate::error::ShiftRegError;