#![no_std]
extern crate alloc;

mod slab;
mod cache;

pub use slab::Slab;
pub use cache::SlabCache;