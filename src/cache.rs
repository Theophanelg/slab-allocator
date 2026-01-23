use crate::Slab;
use alloc::vec::Vec;

pub struct SlabCache{
    object_size: usize,
    slab_size: usize,
    slabs_partial: Vec<Slab>, // slabs avec objets libres
    slabs_full: Vec<Slab>, // slabs pleins
    slabs_free: Vec<Slab>, // slabs vide
}

impl SlabCache {
    pub fn new(object_size: usize, slab_size: usize) -> Self {
        SlabCache{
            object_size,
            slab_size,
            slabs_partial: Vec::new(),
            slabs_full: Vec::new(),
            slabs_free: Vec::new(),
        }
    }

    pub fn allocate(&mut self) -> Option<*mut u8> {
        None
    }
    
    pub unsafe fn deallocate(&mut self, _ptr: *mut u8) {

    }


}