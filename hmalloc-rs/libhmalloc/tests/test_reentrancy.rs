use hmalloc::{hfree, hmalloc};
use std::sync::{Arc, Barrier};
use std::thread;

#[test]
fn test_reentrancy_and_deadlock_regression() {
    // This test ensures that when multiple threads attempt to call hmalloc
    // for the very first time (which triggers OnceCell initialization of AllocState
    // and potentially jemalloc bootstrapping), they do not deadlock.
    
    let num_threads = 20;
    let barrier = Arc::new(Barrier::new(num_threads));
    let mut handles = vec![];

    for _ in 0..num_threads {
        let b = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            // Wait until all threads are ready so we maximize the chance of hitting
            // the OnceCell and jemalloc bootstrap simultaneously.
            b.wait();

            unsafe {
                let ptr = hmalloc(1024);
                assert!(!ptr.is_null());
                hfree(ptr);
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to finish. If there's a deadlock, this will hang.
    for handle in handles {
        handle.join().unwrap();
    }
}
