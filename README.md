# slab-allocator
Slab allocator theory + basic implementation and code integration of the previous project FAT32 implementation

## Documentation 
- [The Slab Allocator - Linux Kernel Documentation](https://www.kernel.org/doc/gorman/html/understand/understand011.html) - Documentation complète sur l'implémentation du slab allocator dans le noyau Linux
- [Slab Allocation - Wikipedia](https://en.wikipedia.org/wiki/Slab_allocation) - Vue d'ensemble du concept de slab allocation
- [Linux SLUB Allocator Internals and Debugging (Oracle)](https://blogs.oracle.com/linux/linux-slub-allocator-internals-and-debugging-1) - Deep-dive sur le SLUB allocator moderne

## Structure

```bash
src /
|--- lib.rs // point d'entrée no std
|--- slab.rs // structure slab avec freelist
```

## Building

```bash
cargo build
cargo test
```


