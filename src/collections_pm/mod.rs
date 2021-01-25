

mod simplekv;
mod btree;
mod vec_with_size;
pub type KvStore<V> = simplekv::KvStore<V>;
pub type Btree = btree::BTree;
pub type VecWithSize = vec_with_size::vec_with_size;

use std::vec::Vec as StdVec;
use corundum::default::*;
type P = BuddyAlloc;

// args[0]: file-name, args[1..]: arguments
// Value: String
// e.g.
//                              args[0]      
//      cargo run vec_with_size file-name state                : print state of pool file
//      cargo run vec_with_size file-name get 4                : get value of vec[4]
//      cargo run vec_with_size file-name put hi               : put value "hi" into vec
//      cargo run vec_with_size file-name burst get 1000       : Get value of vec[0...999]
//      cargo run vec_with_size file-name burst put 1000       : Put value "0"..."999" into vec
//      cargo run vec_with_size file-name burst putget 1000    : burst put 1000 -> burst get 1000
pub fn operate_vec_with_size(args: StdVec<String>) {
    // println!("{}", args.len());
    if args.len() < 2 {
        println!(
            "usage: cargo run file-name [get index|put value] | [burst get|put|putget count] | [state]",
        );
        return;
    }
    
    let root = P::open::<VecWithSize>(&args[0], O_CFNE | O_1GB).unwrap();
    if args[1] == String::from("state") && args.len() == 2 {
        root.print_state();
    }
    
    // get
    if args[1] == String::from("get") && args.len() == 3 {
        println!("{:?}", root.get(args[2].parse().unwrap()))
    }
    // put 
    else if args[1] == String::from("put") && args.len() == 3 {
        root.put(&*args[2])
    }

    // burst
    if args[1] == String::from("burst") {
        // burst put|putget
        if (args[2] == String::from("put") || args[2] == String::from("putget"))
            && args.len() == 4
        {
            for i in 0..args[3].parse().unwrap() {
                root.put(i.to_string().as_str());
                if i == 2000 {
                    // To see what happens when it crashes in the middle of burst
                    // put, uncomment the following line:
                    // panic!("test");
                }
            }
        }

        // burst get|putget
        if (args[2] == String::from("get") || args[2] == String::from("putget"))
            && args.len() == 4
        {
            for i in 0..args[3].parse().unwrap() {
                let v = root.get(i);
                // println!("key:{}", key);
                println!("{:?}", v);
        
            }
        }
    }
}

// args[0]: file-name, args[1..]: arguments
// Key: String, Value: i32
// e.g.
//                        args[0]      
//      cargo run kvstore file-name get key1
//      cargo run kvstore file-name put key1 5
//      cargo run kvstore file-name burst get 1000       : Get value of key0...key999
//      cargo run kvstore file-name burst put 1000       : Put value <key0, 0>, ..., <key999, 999>
//      cargo run kvstore file-name burst putget 1000    : burst put 1000 -> burst get 1000
pub fn operate_KvStore(args: StdVec<String>) {
    if args.len() < 3 {
        println!(
            "usage: cargo run file-name [get key|put key value] | [burst get|put|putget count]",
        );
        return;
    }
    let root = P::open::<KvStore<i32>>(&args[0], O_CFNE | O_1GB).unwrap();

    // get
    if args[1] == String::from("get") && args.len() == 3 {
        println!("{:?}", root.get(&*args[2]))
    }
    // put 
    else if args[1] == String::from("put") && args.len() == 4 {
        root.put(&*args[2], args[3].parse().unwrap())
    }

    // burst
    if args[1] == String::from("burst") {
        // burst put|putget
        if (args[2] == String::from("put") || args[2] == String::from("putget"))
            && args.len() == 4
        {
            for i in 0..args[3].parse().unwrap() {
                let key = format!("key{}", i);
                root.put(&*key, i);
                if i == 2000 {
                    // To see what happens when it crashes in the middle of burst
                    // put, uncomment the following line:
                    // panic!("test");
                }
            }
        }

        // burst get|putget
        if (args[2] == String::from("get") || args[2] == String::from("putget"))
            && args.len() == 4
        {
            for i in 0..args[3].parse().unwrap() {
                let key = format!("key{}", i);
                let v = root.get(&*key);
                // println!("key:{}", key);
                println!("{:?}", v);
        
            }
        }
    }
}