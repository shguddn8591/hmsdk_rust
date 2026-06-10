use libc::{c_void, EINVAL};
use std::ptr;

use hmalloc::{
    haligned_alloc, hcalloc, hfree, hmalloc, hmalloc_usable_size, hposix_memalign, hrealloc,
};

#[test]
fn test_hfree_null() {
    // Calling free on NULL should be a no-op and not segfault.
    unsafe {
        hfree(ptr::null_mut());
    }
}

#[test]
fn test_hmalloc_zero_size() {
    // malloc(0) either returns NULL or a unique pointer that can be passed to free()
    unsafe {
        let ptr = hmalloc(0);
        hfree(ptr);
    }
}

#[test]
fn test_hcalloc_zero_size() {
    unsafe {
        let ptr1 = hcalloc(0, 10);
        let ptr2 = hcalloc(10, 0);
        let ptr3 = hcalloc(0, 0);

        hfree(ptr1);
        hfree(ptr2);
        hfree(ptr3);
    }
}

#[test]
fn test_hrealloc_edge_cases() {
    unsafe {
        // realloc(NULL, size) behaves like malloc(size)
        let ptr = hrealloc(ptr::null_mut(), 100);
        assert!(!ptr.is_null());

        // realloc(ptr, 0) behaves like free(ptr) and returns NULL
        let ptr2 = hrealloc(ptr, 0);
        assert!(ptr2.is_null());

        // realloc(NULL, 0) behaves like malloc(0)
        let ptr3 = hrealloc(ptr::null_mut(), 0);
        hfree(ptr3);
    }
}

#[test]
fn test_hmalloc_usable_size_null() {
    // usable_size(NULL) should safely return 0
    unsafe {
        assert_eq!(hmalloc_usable_size(ptr::null_mut()), 0);
    }
}

#[test]
fn test_hmalloc_out_of_memory() {
    // Requesting an impossible amount of memory should return NULL, not panic
    unsafe {
        let ptr = hmalloc(usize::MAX);
        assert!(ptr.is_null());

        let ptr2 = hcalloc(usize::MAX / 2, usize::MAX / 2);
        assert!(ptr2.is_null());
    }
}

#[test]
fn test_hposix_memalign_edge_cases() {
    unsafe {
        let mut ptr: *mut c_void = ptr::null_mut();

        // Alignment not a power of two
        let ret = hposix_memalign(&mut ptr, 15, 100);
        assert_eq!(ret, EINVAL);

        // Alignment not a multiple of sizeof(void*)
        let ret2 = hposix_memalign(&mut ptr, 2, 100);
        assert_eq!(ret2, EINVAL);

        // Size 0 is valid, should set ptr to something freeable
        let ret3 = hposix_memalign(&mut ptr, std::mem::size_of::<*mut c_void>(), 0);
        assert_eq!(ret3, 0);
        hfree(ptr);
    }
}

#[test]
fn test_haligned_alloc_edge_cases() {
    unsafe {
        // Alignment not a power of two (UB in standard, but jemalloc should handle or return NULL safely)
        let ptr = haligned_alloc(15, 100);
        assert!(ptr.is_null());

        // Size not a multiple of alignment
        let ptr2 = haligned_alloc(64, 63);
        assert!(ptr2.is_null());

        // Valid aligned_alloc
        let ptr3 = haligned_alloc(64, 64);
        assert!(!ptr3.is_null());
        hfree(ptr3);
    }
}
