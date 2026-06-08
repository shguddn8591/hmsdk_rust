use hmalloc::{hfree, hmalloc, hmalloc_usable_size};

#[test]
fn test_large_allocation_cycle() {
    // 50 MB
    let large_size: usize = 50 * 1024 * 1024;

    // Allocate, verify, and free repeatedly to ensure memory is properly
    // returned to the system/jemalloc and reused without causing an OOM.
    for _ in 0..20 {
        unsafe {
            let ptr = hmalloc(large_size);
            assert!(
                !ptr.is_null(),
                "hmalloc returned NULL for a large allocation"
            );

            // Validate the usable size is at least what we requested
            let usable = hmalloc_usable_size(ptr);
            assert!(
                usable >= large_size,
                "Usable size is smaller than requested"
            );

            // Write to the memory to ensure it's physically backed
            // We just write to the beginning, middle, and end to avoid OOM killer from overcommit
            // if we touched every single page, but still verify accessibility.
            let byte_ptr = ptr as *mut u8;
            *byte_ptr = 1;
            *(byte_ptr.add(large_size / 2)) = 2;
            *(byte_ptr.add(large_size - 1)) = 3;

            hfree(ptr);
        }
    }
}
