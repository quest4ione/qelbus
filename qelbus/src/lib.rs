#![no_std]
#[cfg(feature = "alloc")]
extern crate alloc;

mod computer;
pub mod memory;

pub use self::{computer::Computer, memory::Memory};
