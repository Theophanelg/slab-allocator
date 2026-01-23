#![no_std]
extern crate alloc;
use alloc::vec::Vec;

mod slab;
mod cache;

pub use slab::Slab;
pub use cache::SlabCache;