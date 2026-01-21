/// Bloc de mémoire contenant les objets de taille fixe 
pub struct Slab {
    memory: *mut u8, /// Début du slab
    object_size: usize, /// taille des objets
    capacity: usize, /// nombre total des objets
    freelist: *mut u8, /// premier objet libre
    free_count: usize,  /// nombre object libre
}

impl Slab {
    /// Créer un nouveau slab via un bloc de mémoire
    /// 
    /// # Safety
    /// Le pointeur `memory` doit être valide et de taille `total_size`
    pub unsafe fn new(memory: *mut u8, object_size: usize, total_size: usize) -> Self {
        let capacity = total_size / object_size;
        let mut i = 0;
        while i < capacity {
            let current_object = memory.add(i * object_size);
            
            if i < capacity - 1 {
                let nxt_object = memory.add((i + 1) * object_size);
                let ptr_object = current_object as *mut *mut u8;
                *ptr_object = nxt_object; 
            } else {
                let ptr_object = current_object as *mut *mut u8;
                *ptr_object = core::ptr::null_mut();
            }

            i = i + 1;
        }
        Slab {
            memory: memory,
            object_size: object_size,
            capacity: capacity,
            freelist: memory,
            free_count: capacity,
        }
    }

    
}