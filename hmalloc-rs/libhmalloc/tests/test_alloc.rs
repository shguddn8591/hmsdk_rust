use libc::{c_void, size_t};

extern "C" {
    fn hmalloc(size: size_t) -> *mut c_void;
    fn hfree(ptr: *mut c_void);
    fn hcalloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn hrealloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn hmalloc_usable_size(ptr: *mut c_void) -> size_t;
}

#[test]
fn hmalloc_basic() {
    unsafe {
        let ptr = hmalloc(1024);
        assert!(!ptr.is_null());
        hfree(ptr);
    }
}

#[test]
fn hmalloc_zero() {
    unsafe {
        let ptr = hmalloc(0);
        hfree(ptr);
        // zero-size alloc: jemalloc returns non-null, libc may return null; either is ok
    }
}

#[test]
fn hmalloc_large() {
    unsafe {
        let ptr = hmalloc(500 * 1024 * 1024);
        assert!(!ptr.is_null());
        hfree(ptr);
    }
}

#[test]
fn hcalloc_zeroed() {
    unsafe {
        let ptr = hcalloc(100, 8) as *mut u8;
        assert!(!ptr.is_null());
        for i in 0..800 {
            assert_eq!(*ptr.add(i), 0, "byte {} not zero", i);
        }
        hfree(ptr as *mut c_void);
    }
}

#[test]
fn hrealloc_grow() {
    unsafe {
        let ptr = hmalloc(128);
        assert!(!ptr.is_null());
        let ptr2 = hrealloc(ptr, 4096);
        assert!(!ptr2.is_null());
        hfree(ptr2);
    }
}

#[test]
fn hrealloc_null_nonzero() {
    unsafe {
        let ptr = hrealloc(std::ptr::null_mut(), 256);
        assert!(!ptr.is_null());
        hfree(ptr);
    }
}

#[test]
fn hrealloc_nonnull_zero() {
    unsafe {
        let ptr = hmalloc(64);
        assert!(!ptr.is_null());
        let ptr2 = hrealloc(ptr, 0);
        // result is null after free via realloc(ptr, 0)
        assert!(ptr2.is_null());
    }
}

#[test]
fn hmalloc_usable_size_nonnull() {
    unsafe {
        let ptr = hmalloc(100);
        assert!(!ptr.is_null());
        let sz = hmalloc_usable_size(ptr);
        assert!(sz >= 100);
        hfree(ptr);
    }
}

#[test]
fn hmalloc_usable_size_null() {
    unsafe {
        let sz = hmalloc_usable_size(std::ptr::null_mut());
        assert_eq!(sz, 0);
    }
}
