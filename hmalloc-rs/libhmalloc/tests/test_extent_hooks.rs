use hmalloc::{hfree, hmalloc, hrealloc};

#[test]
fn test_jemalloc_extent_hooks_fragmentation() {
    // Allocate various chunk sizes to trigger extent hook split/merge behavior
    // jemalloc manages extents dynamically. If our custom hooks are broken,
    // varied sizes of allocs and frees will cause memory corruption or crashes.
    let mut ptrs = Vec::new();

    // 1. Allocate 2MB, 4MB, 8MB, 16MB
    let sizes = [
        2 * 1024 * 1024,
        4 * 1024 * 1024,
        8 * 1024 * 1024,
        16 * 1024 * 1024,
    ];

    for &size in &sizes {
        unsafe {
            let ptr = hmalloc(size);
            assert!(!ptr.is_null());
            
            // Touch it
            let byte_ptr = ptr as *mut u8;
            *byte_ptr = 42;
            *(byte_ptr.add(size - 1)) = 42;

            ptrs.push((ptr, size));
        }
    }

    // 2. Free the middle extents (4MB and 8MB) to create a hole
    unsafe {
        hfree(ptrs[1].0);
        hfree(ptrs[2].0);
    }

    // 3. Allocate a 6MB chunk which should fit in the hole (causing an extent split/merge internally)
    unsafe {
        let ptr_6mb = hmalloc(6 * 1024 * 1024);
        assert!(!ptr_6mb.is_null());
        
        let byte_ptr = ptr_6mb as *mut u8;
        *byte_ptr = 99;
        *(byte_ptr.add((6 * 1024 * 1024) - 1)) = 99;

        hfree(ptr_6mb);
    }

    // 4. Reallocate the 2MB chunk to 10MB
    unsafe {
        let ptr_realloc = hrealloc(ptrs[0].0, 10 * 1024 * 1024);
        assert!(!ptr_realloc.is_null());
        
        let byte_ptr = ptr_realloc as *mut u8;
        *byte_ptr = 77;
        *(byte_ptr.add((10 * 1024 * 1024) - 1)) = 77;

        hfree(ptr_realloc);
    }

    // 5. Clean up the last 16MB extent
    unsafe {
        hfree(ptrs[3].0);
    }
}
