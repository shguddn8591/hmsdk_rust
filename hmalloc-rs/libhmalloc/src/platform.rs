use libc::{c_int, c_void, size_t};

pub unsafe fn errno() -> c_int {
    #[cfg(target_os = "linux")]
    return *libc::__errno_location();
    #[cfg(target_os = "macos")]
    return *libc::__error();
}

pub unsafe fn set_errno(e: c_int) {
    #[cfg(target_os = "linux")]
    {
        *libc::__errno_location() = e;
    }
    #[cfg(target_os = "macos")]
    {
        *libc::__error() = e;
    }
}

pub unsafe fn system_malloc_usable_size(ptr: *mut c_void) -> size_t {
    #[cfg(target_os = "linux")]
    return libc::malloc_usable_size(ptr);
    #[cfg(not(target_os = "linux"))]
    {
        let _ = ptr;
        return 0;
    }
}
