# Test Corundum
At First, How Corundum recover pool file from inconsistency?
    
In `BuddyAlloc` that author provides as default implementing `MemPool` trait, `BuddyAlloc::recover()` is called when open pool file with `BuddyAlloc::open()`.
(More precisely, `recover()` is called from `open_with_no_root()`)

So, the pool file is recovered from unconsistent state when open file using `BuddyAlloc::open()`.

## TODO
- Fill below
- Parsing argument: use `clap` crate with yaml
- Test PMDK in this repository as well?

## How to test each Corundum collections
---
### SimpleKV
- todo

### BTree
- todo

## How to break consistency of pool file directly
---
- todo