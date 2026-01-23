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
        if let Some(slab) = self.slabs_partial.first_mut() {
            if let Some(obj) = slab.allocate(){
                if slab.is_full() {
                    let slab = self.slabs_partial.remove(0);
                    self.slabs_full.push(slab);
                }
                return Some(obj);
            }
        }

        if let Some(mut slab) = self.slabs_free.pop(){
            let obj = slab.allocate();
            self.slabs_partial.push(slab);
            return obj;
        }

        None
    }
    
    pub unsafe fn deallocate(&mut self, _ptr: *mut u8) {

    }


}