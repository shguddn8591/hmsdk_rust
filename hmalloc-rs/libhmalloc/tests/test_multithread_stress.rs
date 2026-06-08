use hmalloc::{hfree, hmalloc};
use std::thread;

const NUM_THREADS: usize = 100;
const ALLOCS_PER_THREAD: usize = 1000;
const ALLOC_SIZE: usize = 256; // 256 bytes

#[test]
fn test_multithread_stress() {
    let mut handles = vec![];

    // Spawn multiple threads, each doing many allocations and frees
    for _ in 0..NUM_THREADS {
        let handle = thread::spawn(|| {
            let mut ptrs = Vec::with_capacity(ALLOCS_PER_THREAD);

            // Allocate
            for _ in 0..ALLOCS_PER_THREAD {
                unsafe {
                    let ptr = hmalloc(ALLOC_SIZE);
                    assert!(!ptr.is_null(), "hmalloc returned NULL during stress test");
                    ptrs.push(ptr);
                }
            }

            // Free
            for ptr in ptrs {
                unsafe {
                    hfree(ptr);
                }
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}
