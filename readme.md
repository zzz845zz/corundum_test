# Test Corundum
At First, How Corundum recover pool file from inconsistency?
    
In `BuddyAlloc` that author provides as default implementing `MemPool` trait, `BuddyAlloc::recover()` is called when open pool file with `BuddyAlloc::open()`.
(More precisely, `recover()` is called from `open_with_no_root()`)

So, the pool file is recovered from unconsistent state when open file using `BuddyAlloc::open()`.

## TODO
- Implement 
  + Refer to [paper](https://drive.google.com/file/u/1/d/1j9bz4OjnY3iY-LWhQlYwAbmL7F_iZ2wP/view?usp=sharing)
  + Detectable execution(3.2 & 5)
  + Durable Queue(3.1 & 4), Log Queue(3.2 & 5)
- Readme 정리?
- Script for test (e.g. script automation to replace [these manual copy&paste command](https://github.com/zzz845zz/corundum_test/blob/master/scripts/test_vec_with_size_recovery.sh))
- Parsing argument: use `clap` crate with yaml
- Test PMDK in this repository as well?

## 1. How to test each Corundum collections
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
## 2. How to test recovery of Corundum 
### Control and Test using environment variable
- `no_recover=1` : Turn off recovery function when opening pool file
- `crash_put=1` : crash during `put()` operation (Only applies to `VecWithSize`)
- `crash_del=1`: crash during `del()` operation (Only applies to `VecWithSize`)

```bash
# Examples
no_recover=1 cargo run vec_with_size ./src/test_vec_with_size.pool state # open pool without recovery and show status
no_recover=1 cargo run vec_with_size ./src/test_vec_with_size.pool put e0 # open pool without recovery and `put e0`
no_recover=1 crash_put=1 cargo run vec_with_size ./src/test_vec_with_size.pool put e1 # open pool without recovery, process will be crashed during `put` operation
crash_del=1 cargo run vec_with_size ./src/test_vec_with_size.pool del e1 # open pool with recovery, process will be crashed during `del` operation
```

## 3. Experiments
### VecWithSize
  + `size`, `elements`를 가진 array
  + 기대사항: `size = len(elements)`
  + operation(with transaction)
    + `state`: pool file 상태 출력(size, element, consistent state(size==len(element)))
    + `put e0`: size+=1 -> (crashed or not) -> add element "e0"
    + `del 0`: size-=1 -> (crashed or not) -> delete element at index 0
    + `get 0`: get element at index 0
- Recovery test를 위한 부가 기능
    + crash injection: 위 transaction 중간에 프로세스 종료(crashed or not)
      +  `crash_put=1`, `crash_del=1`
    + recovery on/off
      + `no_recover=1`
- Recovery test
  + 실험환경: `cp-service.kaist.ac.kr`, Single thread, 1GB pool file
  + [command](https://github.com/zzz845zz/corundum_test/blob/master/scripts/test_vec_with_size_recovery.sh) -> [output](https://github.com/zzz845zz/corundum_test/blob/master/scripts/test_vec_with_size_recovery_output.txt)
    ```bash
    # Recovery test example
    create & state # size 0, elems: [], state: consistent
    put e0
    put e1
    put e2 (crashed)
    state (no recovery) # size 3, elems: [e0, e1], state: inconsistent
    state (recovery) # size 2, elems: [e0, e1], state: consistent
    put e2
    delete 2 (crashed)
    state (no recovery) # size 2, elems: [e0, e1, e2], state: inconsistent
    state (recovery) # size 3, elems: [e0, e1, e2], state: consistent
    ```

### Others
- todo