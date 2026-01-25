#[test]
fn test_slab_allocate_and_free(){
    let mut buffer = vec![0u8;1024];
    let ptr = buffer.as_mut_ptr();

    unsafe {
        // SAFETY: ptr provient de as_mut_ptr qui est valide
        // taille 256 bytes, object_size 64 qui fait 4 objets possibles
        let mut slab = slab_allocator::Slab::new(ptr,64,256);
        assert_eq!(slab.is_empty(), true);
        assert_eq!(slab.is_full(), false);

        let obj1 = slab.allocate().expect("Should allocate obj1");
        let obj2 = slab.allocate().expect("Should allocate obj2");
        
        assert_ne!(obj1, obj2);
        assert_eq!(slab.is_empty(), false);

        slab.deallocate(obj1);

        let obj3 = slab.allocate().expect("Should allocate obj3");
        assert_eq!(obj1, obj3);

        slab.deallocate(obj2);
        slab.deallocate(obj3);
        
        assert_eq!(slab.is_empty(), true)
    }
}

#[test]
fn test_slab_full() {
    let mut buffer = vec![0u8; 256];
    let ptr = buffer.as_mut_ptr();
    unsafe{
        // SAFETY: ptr provient de as_mut_ptr qui est valide
        // taille 256 bytes, object_size 64 qui fait 4 objets possibles
        let mut slab = slab_allocator::Slab::new(ptr, 64, 256);
        let _obj1 = slab.allocate();
        let _obj2 = slab.allocate();
        let _obj3 = slab.allocate();
        let _obj4 = slab.allocate();

        assert_eq!(slab.is_full(), true);
        let obj5 = slab.allocate();
        assert_eq!(obj5.is_none(), true);
    }
}