
use corundum::alloc::MemPool;
use corundum::alloc::*;
use corundum::default::BuddyAlloc;
use std::alloc::{alloc, dealloc, realloc, Layout};


struct TrackAlloc {}

unsafe impl MemPool for TrackAlloc {

    unsafe fn pre_alloc(size: usize) -> (*mut u8, u64, usize, usize) {
        let p = alloc(Layout::from_size_align_unchecked(size, 4));
        println!("A block of {} bytes is allocated at {}", size, p as u64);
        (p, p as u64, size, 0)
    }
    unsafe fn pre_dealloc(p: *mut u8, size: usize) -> usize {
        println!("A block of {} bytes at {} is deallocated", size, p as u64);
        dealloc(p, Layout::from_size_align_unchecked(size, 1));
        0
    }
}

#[test]
fn test_pool() {
    unsafe {
        let a = TrackAlloc::open::<i32>("test.pool", O_CF).unwrap();

        let (p, _, _) = TrackAlloc::alloc(1);
        *p = 10;
        println!("loc {} contains {}", p as u64, *p);
        TrackAlloc::dealloc(p, 1);
    }
}