
use corundum::default::*;
use corundum::boxed::Pbox;
use corundum::prc::*;
use corundum::cell::*;
use corundum::stm::*;
use corundum::stm::Journal;
use corundum::sync::*;
use corundum::vec::Vec;

use std::{mem::replace, thread};

type H1 = Heap;
type H2 = Heap;

#[test]
fn test_heap() {
    H1::transaction(|j| {
        let mut vec = Vec::from_slice(&[1, 2], j);
        println!("--- Initialize ---");
        for x in &*vec {
            println!("{}", x);
        }
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], 1);
    
        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.len(), 1);
        
        vec[0] = 7;
        assert_eq!(vec[0], 7);
        
        vec.extend_from_slice(&[1, 2, 3], j);
        
        println!("--- After insert ---");
        for x in &*vec {
            println!("{}", x);
        }

        // H2::transaction(|j2| {
        //     vec.insert(0, -5, j);
        //     // vec[1]= 5;
            
        // }).unwrap();
        
        println!("--- After rollback ---");
        unsafe { Heap::rollback() };
        for x in &*vec {
            println!("{}", x);
        }
        // assert_eq!(vec, [7, 1, 2, 3]);
        // assert_eq!(vec, [1, 2]);
    }).unwrap();
}

#[test]
fn test_heap2() {
    let root = Heap::open::<i32>("./i8.pool", O_CF).unwrap();
    Heap::print_info();
    let a = Heap::transaction(|j| {
        // let a = root.abs();
        // println!("{}", a);
        let a = *root;
        root.abs()
    }).unwrap();
    println!("{}", a);
    println!("{}", Heap::available());

}


#[test]
fn test_buddyalloc() {
    // let image = B::open_no_root("testpool.pool", O_CF).unwrap();
    type P = BuddyAlloc;
    use corundum::prc::Prc;
    use corundum::clone::PClone;
    use std::mem;

    let head = P::open::<i8>("./testpool.pool", O_CF).unwrap();
    // unsafe { let b = P::alloc(74767675); };
    let a = mem::size_of::<i8>();
    let b = mem::size_of::<P>();
    
    let c = 1;
    unsafe { P::recover() };
    P::transaction(|j| {
        let p = Prc::<i8,P>::new(1, j);

        // Create a new persistent strong reference
        // let s = p.pclone(j);
    
        // assert_eq!(*p, *s);
        // assert_eq!(2, Prc::strong_count(&p));
        // assert_eq!(0, Prc::weak_count(&p));
    
        // // Create a new persistent weak reference
        // let w = Prc::downgrade(&p, j);
        // assert_eq!(2, Prc::strong_count(&p));
        // assert_eq!(1, Prc::weak_count(&p));
    
        // // Create a new volatile weak reference
        // let v = Prc::volatile(&p);
        // assert_eq!(2, Prc::strong_count(&p));
        // assert_eq!(1, Prc::weak_count(&p));
    
        // // Upgrade the persistent weak ref to a strong ref
        // let ws = w.upgrade(j).unwrap();
        // assert_eq!(3, Prc::strong_count(&p));
        // assert_eq!(1, Prc::weak_count(&p));
    
        // // Upgrade the volatile weak ref to a strong ref
        // let vs = w.upgrade(j).unwrap();
        // assert_eq!(4, Prc::strong_count(&p));
        // assert_eq!(1, Prc::weak_count(&p));
    }).unwrap();
}

// #[test]
// fn test_heap() {
//     use std::clone::Clone as StdClone;
//     use std::sync::mpsc::channel;
//     const N: usize = 10;


//     Heap::transaction(|j| {
//         // Spawn a few threads to increment a shared variable (non-atomically), and
//         // let the main thread know once all increments are done.
//         //
//         // Here we're using an Arc to share memory among threads, and the data inside
//         // the Arc is protected with a mutex.
//         let data = Parc::new(Mutex::new(0, j), j);
//         let (tx, rx) = channel();
//         for _ in 0..N {
//             let (data, tx) = (data.demote(), tx.clone());
//             thread::spawn(move || {
//                 // The shared state can only be accessed once the lock is held.
//                 // Our non-atomic increment is safe because we're the only thread
//                 // which can access the shared state when the lock is held.
//                 //
//                 // We unwrap() the return value to assert that we are not expecting
//                 // threads to ever fail while holding the lock.
//                 let res = Heap::transaction(|j| {
//                     let data = data.promote(j).unwrap();
//                     let mut data = data.lock(j);
//                     *data += 1;
//                     *data
//                 })
//                 .unwrap();
//                 // the lock is unlocked here when the transaction commits.


//                 if res == N {
//                     tx.send(()).unwrap();
//                 }
//             });
//         }
//         rx.recv().unwrap();
//     })
//     .unwrap();
// }