# Test Corundum
At First, How Corundum recover pool file from inconsistency?
    
In `BuddyAlloc` that author provides as default implementing `MemPool` trait, `BuddyAlloc::recover()` is called when open pool file with `BuddyAlloc::open()`.
(More precisely, `recover()` is called from `open_with_no_root()`)

So, the pool file is recovered from unconsistent state when open file using `BuddyAlloc::open()`.

## TODO
- Fix input:`cargo run vec_with_size ./src/test_vec_with_size.pool get 1`
    + elements: `[hi, wow]`
    + wrong output: `Some("[hi, wow]")`
    + correct output: `Some("wow")`
- Test recovery of VecWithSize
- Fill below
- Parsing argument: use `clap` crate with yaml
- Test PMDK in this repository as well?

## 1. How to test each Corundum collections
---
### SimpleKV
```bash
cargo run kvstore file-name get key1
cargo run kvstore file-name put key1 5
cargo run kvstore file-name burst get 1000       # Get value of key0...key999
cargo run kvstore file-name burst put 1000       # Put value <key0, 0>, ..., <key999, 999>
cargo run kvstore file-name burst putget 1000    # burst put 1000 -> burst get 1000
```

### BTree
- todo

### VecWithSize
```bash
cargo run vec_with_size file-name state                # print state of pool file
cargo run vec_with_size file-name get 4                # get value of vec[4]
cargo run vec_with_size file-name put hi               # put value "hi" into vec
cargo run vec_with_size file-name burst get 1000       # Get value of vec[0...999]
cargo run vec_with_size file-name burst put 1000       # Put value "0"..."999" into vec
cargo run vec_with_size file-name burst putget 1000    # burst put 1000 -> burst get 1000
```
## 2. How to break consistency of pool file directly
---
- todo

### MEMO
- Arugment로 1, 2 둘다 가능하게끔?