use crate::{Slab};
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
        if let Some(slab) = self.slabs_partial.first_mut()
            && let Some(obj) = slab.allocate(){
                if slab.is_full() {
                    let slab = self.slabs_partial.remove(0);
                    self.slabs_full.push(slab);
                }
                return Some(obj);
            }

        if let Some(mut slab) = self.slabs_free.pop(){
            let obj = slab.allocate();
            self.slabs_partial.push(slab);
            return obj;
        }

        use alloc::alloc::{alloc,Layout};
        let layout = Layout::from_size_align(self.slab_size, 8).unwrap();

        // SAFETY: alloc retourne un pointeur aligné ou null
        let memory = unsafe {alloc(layout)};

        if memory.is_null(){
            return None;
        }

        let mut new_slab = unsafe {
            Slab::new(memory,self.object_size, self.slab_size)
        };
        let obj = new_slab.allocate();
        self.slabs_partial.push(new_slab);
        obj
    }
    

    /// Libérer un object
    /// # Safety
    /// ptr doit provenir de ce cache
    pub unsafe fn deallocate(&mut self, ptr: *mut u8) {
        let _i = 0;
        if let Some(slab) = self.slabs_full.first_mut() {
            // SAFETY: ptr provient de ce cache
            unsafe{
                slab.deallocate(ptr);
            }
            let slab = self.slabs_full.remove(0);
            self.slabs_partial.push(slab);
            return;
        }

        if let Some(slab) = self.slabs_partial.first_mut() {
        
        // SAFETY: ptr provient de ce cache 
        unsafe {
            slab.deallocate(ptr);
        }

        if slab.is_empty() {
            let slab = self.slabs_partial.remove(0);
            self.slabs_free.push(slab);
        }
        }
    }


}