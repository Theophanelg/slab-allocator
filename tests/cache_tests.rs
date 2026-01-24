#[test]
fn test_cache(){
    let mut cache = slab_allocator::SlabCache::new(64,256);
    unsafe {
        let obj1 = cache.allocate().expect("should allocate");
        let obj2 = cache.allocate().expect("should allocate");

        cache.deallocate(obj1);
        cache.deallocate(obj2);
    }
}