use corundum::alloc::*;
use corundum::*;
use corundum::default::*;


use std::{panic::{RefUnwindSafe, UnwindSafe}, vec};
use std::env;
use std::fmt::{Display, Error, Formatter};


type P = BuddyAlloc;
type Pbox<T> = corundum::boxed::Pbox<T, P>;
// pub struct vec_with_size<V: PSafe> {
//     size: Parc<PMutex<usize>>,
//     elements: Parc<PMutex<PVec<V>>>,
// }

// #[derive(Root)]
pub struct vec_with_size {
    size: Parc<PMutex<usize>>,
    elements: Parc<PMutex<PVec<PString>>>,
}

// impl<V:PSafe> RootObj<P> for vec_with_size<V> {
impl RootObj<P> for vec_with_size {
    fn init(j: &Journal) -> Self {
        vec_with_size {
            size: Parc::new(PMutex::new(0, j), j),
            elements: Parc::new(PMutex::new(PVec::new(j), j), j),
        }
    }
}

// impl<V:PSafe + Copy> vec_with_size<V> 
// where
//     V: TxInSafe + TxOutSafe + RefUnwindSafe
// {
impl vec_with_size {
    pub fn put(&self, value: &str) {
        P::transaction(|j| {
            *self.size.lock(j) += 1;
            self.elements.lock(j).push(PString::from_str(value, j), j);
        }).unwrap();
    }

    pub fn get(&self, index: usize) -> Option<String> {
        P::transaction(|j| {
            let mut size = self.size.lock(j);
            if index >= *size {
                return None;
            }
            Some(self.elements.lock(j).to_string())
        }).unwrap()

    }

    pub fn delete(&self, index: usize) -> Option<String> {
        P::transaction(|j| {
            let mut size = self.size.lock(j);
            if index >= *size {
                return None;
            }
            *size -= 1;
            Some(self.elements.lock(j).remove(index).to_string())
        }).unwrap()
    }

    pub fn print_state(&self) {
        let (size, elements_string, state) = P::transaction(|j| {
            let size = *self.size.lock(j);
            let elements = self.elements.lock(j);
            let elements_string = elements.iter()
                .map(|pstr| pstr.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            let state = size == elements.len();
            (size, elements_string, state)
        }).unwrap();

        println!(
            "size: {}\nelements: {}\nstate:{}",
            size, elements_string, state
        )
    }
}

#[test]
fn test() {
    use std::env;
    use std::vec::Vec as StdVec;
    
    let mut args: StdVec<String> = env::args().collect();
    args.remove(args.len()-1); // remove 
    args.remove(0); // remove "--nocapture"
    
    if args.len() < 3 {
        println!(
            "usage: {} file-name [get index|put value] | [burst get|put|putget count] | [state]",
            args[0]
        );
        return;
    }
    for (i, arg) in args.iter().enumerate() {
        println!("arg {}:{}", i, arg);
    }
    
    let root = P::open::<vec_with_size>(&args[1], O_CFNE | O_1GB).unwrap();
    if args[2] == String::from("state") && args.len() == 3 {
        root.print_state();
    }
    
    if args[2] == String::from("get") && args.len() == 4 {
        println!("{:?}", root.get(args[3].parse().unwrap()))
    } else if args[2] == String::from("put") && args.len() == 4 {
        root.put(&*args[3])
    }
    if args[2] == String::from("burst")
        && (args[3] == String::from("put") || args[3] == String::from("putget"))
        && args.len() == 5
    {
        for i in 0..args[4].parse().unwrap() {
            let key = format!("key{}", i);
            root.put(&*key);
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
            let v = root.get(i);
            // println!("key:{}", key);
            println!("{:?}", v);

        }
    }
}

