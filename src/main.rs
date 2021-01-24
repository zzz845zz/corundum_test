use corundum::default::*;
type P = BuddyAlloc;

mod collections;
use collections::KvStore;
use collections::Btree;

pub fn main() {
    use std::env;
    use std::vec::Vec as StdVec;

    let args: StdVec<String> = env::args().collect();

    if args.len() < 3 {
        println!(
            "usage: {} file-name [get key|put key value] | [burst get|put|putget count]",
            args[0]
        );
        return;
    }

    for (i, arg) in args.iter().enumerate() {
        println!("arg {}:{}", i, arg);
    }
    let root = P::open::<KvStore<i32>>(&args[1], O_CFNE | O_1GB).unwrap();

    if args[2] == String::from("get") && args.len() == 4 {
        println!("{:?}", root.get(&*args[3]))
    } else if args[2] == String::from("put") && args.len() == 5 {
        root.put(&*args[3], args[4].parse().unwrap())
    }
    if args[2] == String::from("burst")
        && (args[3] == String::from("put") || args[3] == String::from("putget"))
        && args.len() == 5
    {
        for i in 0..args[4].parse().unwrap() {
            let key = format!("key{}", i);
            root.put(&*key, i);
            if i == 2000 {
                // To see what happens when it crashes in the middle of burst
                // put, uncomment the following line:
                // panic!("test");
            }
        }
    }
    if args[2] == String::from("burst")
        && (args[3] == String::from("get") || args[3] == String::from("putget"))
        && args.len() == 5
    {
        for i in 0..args[4].parse().unwrap() {
            let key = format!("key{}", i);
            root.get(&*key);
            // println!("key:{}", key);
            // println!("{:?}", root.get(&*key));

        }
    }
}
