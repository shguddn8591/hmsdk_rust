use hmalloc::{hmmap, hmunmap};

#[test]
fn hmmap_anonymous() {
    unsafe {
        let len = 4096usize;
        let ptr = hmmap(
            std::ptr::null_mut(),
            len,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANON,
            -1,
            0,
        );
        assert_ne!(ptr, libc::MAP_FAILED, "mmap failed");
        assert!(!ptr.is_null());

        // Write and read back
        let slice = std::slice::from_raw_parts_mut(ptr as *mut u8, len);
        slice[0] = 42;
        assert_eq!(slice[0], 42);

        let ret = hmunmap(ptr, len);
        assert_eq!(ret, 0);
    }
}

#[test]
fn hmmap_large() {
    unsafe {
        let len = 2 * 1024 * 1024usize; // 2 MiB
        let ptr = hmmap(
            std::ptr::null_mut(),
            len,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANON,
            -1,
            0,
        );
        assert_ne!(ptr, libc::MAP_FAILED);
        let ret = hmunmap(ptr, len);
        assert_eq!(ret, 0);
    }
}
