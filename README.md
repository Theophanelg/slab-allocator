# slab-allocator
Slab allocator theory + basic implementation and code integration of the previous project FAT32 implementation

## Objectif du Projet
Ce projet répond à deux objectifs :
1. **Implémentation** d'un allocateur slab minimal en Rust (no_std)
2. **Documentation technique** du SLUB allocator Linux pour la préparation à la classe d'exploitation

## Fonctionnalités Implémentées
- ✅ Structure `Slab` avec freelist LIFO (Last-In First-Out)
- ✅ `SlabCache` gérant plusieurs slabs (full/partial/free)
- ✅ Allocation et libération d'objets
- ✅ Création automatique de nouveaux slabs avec `alloc::alloc()`
- ✅ Tests unitaires complets
- ✅ Documentation `unsafe` complète

## Architecture
```bash
src/
├── lib.rs # Point d'entrée no_std
├── slab.rs # Structure Slab + freelist
└── cache.rs # SlabCache

tests/
├── slab_basics.rs # Tests Slab
└── cache_tests.rs # Tests SlabCache
```

## Build et Tests

```bash
# Compilation
cargo build

# Tests
cargo test

# Documentation
cargo doc --no-deps --open

# Qualité
cargo clippy -- -D warnings
```

## Documentation 
- [The Slab Allocator - Linux Kernel Documentation](https://www.kernel.org/doc/gorman/html/understand/understand011.html)
- [Slab Allocation - Wikipedia](https://en.wikipedia.org/wiki/Slab_allocation)
- [Linux SLUB Allocator Internals (Oracle)](https://blogs.oracle.com/linux/linux-slub-allocator-internals-and-debugging-1)

### Implémentation Rust
- [slaballoc crate](https://docs.rs/slaballoc)
- [Rust unsafe code guidelines](https://doc.rust-lang.org/nomicon/)
- [Learning Rust with Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)

### Exploitation
- [Kernel Heap Exploitation](https://argp.github.io/2012/01/03/linux-kernel-heap-exploitation/)
- [CVE-2017-11176 Exploitation](https://blog.lexfo.fr/cve-2017-11176-linux-kernel-exploitation-part3.html)

### Writeup Technique
Voir [SLUB_WRITEUP.md](SLUB_WRITEUP.md) pour :
- Architecture SLUB (kmem_cache, slabs, freelists)
- Mécanismes d'allocation/libération
- Techniques d'exploitation (UAF, double-free, slab spraying)



