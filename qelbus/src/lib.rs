#![cfg_attr(not(feature = "std"), no_std)]
mod computer;
pub mod memory;

pub use self::{computer::Computer, memory::Memory};
