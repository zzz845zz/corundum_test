mod collections_pm;
use collections_pm::*;

use std::env;
use std::vec::Vec as StdVec;

fn print_usage(args: StdVec<String>) {
    println!(
        "usage: {} collection file-name args...[get key|put key value] | [burst get|put|putget count]",
        args[0]
    );
}

pub fn main() {
    let mut args: StdVec<String> = env::args().collect();

    // for (ix, arg) in args.iter().enumerate() {
    //     println!("{}: {}", ix, arg);
    // }

    if args.len() < 3 {
        print_usage(args);
        return;
    }
    args.remove(0);
    let collection = args.remove(0);
    match collection.as_str() {
        "kvstore" => operate_KvStore(args),
        "vec_with_size" => operate_vec_with_size(args),
        _ => {
            print_usage(args);
            return;
        }
    }
}
