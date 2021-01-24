use corundum::default::*;
use std::{process::exit, thread};

#[test]
fn parc_read() {

    let p = BuddyAlloc::open::<Parc<PMutex<i32>>>("./tests/foo.pool", O_CFNE).unwrap();
    let v = p.volatile();
    
    transaction(|j| {
        let p = v.clone();
        if let Some(p) = p.upgrade(j) {
            println!("value is {}", *p.lock(j));
        }
    }).unwrap();
}

#[test]
fn parc_write() {

    let p = BuddyAlloc::open::<Parc<PMutex<i32>>>("./tests/foo.pool", O_CFNE).unwrap();
    let v = p.volatile();
     
    let p = v.clone();
    transaction(|j| {
        if let Some(p) = p.upgrade(j) {
            *p.lock(j) += 1;
            println!("value after write is {}", *p.lock(j));
            exit(0);
        }
    }).unwrap();
}

#[test]
fn parc_read_multi() {

    let p = BuddyAlloc::open::<Parc<PMutex<i32>>>("./tests/foo.pool", O_CFNE).unwrap();
    let v = p.volatile();
    let mut threads = vec![];
     
    for i in 0..10 {
        let p = v.clone();
        threads.push(thread::spawn(move || {
            transaction(|j| {
                if let Some(p) = p.upgrade(j) {
                    println!("access {} from thread {}", *p.lock(j), i);
                }
            }).unwrap();
        }));
    }
     
    for t in threads {
        t.join().unwrap();
    }
}

#[test]
fn parc_write_multi() {

    let p = BuddyAlloc::open::<Parc<PMutex<i32>>>("./tests/foo.pool", O_CFNE).unwrap();
    let v = p.volatile();
    let mut threads = vec![];
     
    for i in 0..7 {
        let p = v.clone();
        threads.push(thread::spawn(move || {
            transaction(|j| {
                if let Some(p) = p.upgrade(j) {
                    *p.lock(j) += 1;
                }
            }).unwrap();
        }));
    }
     
    for t in threads {
        t.join().unwrap();
    }
}