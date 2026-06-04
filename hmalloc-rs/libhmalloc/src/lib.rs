mod alloc;
mod env;
mod jemalloc;
mod numa;
mod platform;
mod state;

use libc::{c_int, c_void, off_t, size_t};
use tikv_jemalloc_sys as je;

// Ensure initialization at .so load time (LD_PRELOAD safety)
#[cfg(target_os = "linux")]
#[used]
#[link_section = ".init_array"]
static INIT: unsafe extern "C" fn() = {
    unsafe extern "C" fn f() {
        let _ = state::get();
    }
    f
};

#[no_mangle]
pub unsafe extern "C" fn hmalloc(size: size_t) -> *mut c_void {
    let s = state::get();
    if !s.use_jemalloc {
        return libc::malloc(size);
    }
    je::mallocx(size, jemalloc::mallocx_flags(s.arena_index))
}

#[no_mangle]
pub unsafe extern "C" fn hfree(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    let s = state::get();
    if !s.use_jemalloc {
        libc::free(ptr);
        return;
    }
    je::dallocx(ptr, jemalloc::mallocx_flags(s.arena_index));
}

#[no_mangle]
pub unsafe extern "C" fn hcalloc(nmemb: size_t, size: size_t) -> *mut c_void {
    let total = match nmemb.checked_mul(size) {
        Some(n) => n,
        None => return std::ptr::null_mut(),
    };
    let ptr = hmalloc(total);
    if !ptr.is_null() {
        libc::memset(ptr, 0, total);
    }
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn hrealloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    let s = state::get();
    if !s.use_jemalloc {
        return libc::realloc(ptr, size);
    }
    if ptr.is_null() {
        return hmalloc(size);
    }
    if size == 0 {
        hfree(ptr);
        return std::ptr::null_mut();
    }
    je::rallocx(ptr, size, jemalloc::mallocx_flags(s.arena_index))
}

#[no_mangle]
pub unsafe extern "C" fn haligned_alloc(alignment: size_t, size: size_t) -> *mut c_void {
    let s = state::get();
    if !s.use_jemalloc {
        return libc::aligned_alloc(alignment, size);
    }
    if alignment == 0 || !alignment.is_power_of_two() {
        platform::set_errno(libc::EINVAL);
        return std::ptr::null_mut();
    }
    je::mallocx(size, jemalloc::mallocx_align_flags(s.arena_index, alignment))
}

#[no_mangle]
pub unsafe extern "C" fn hposix_memalign(
    memptr: *mut *mut c_void,
    alignment: size_t,
    size: size_t,
) -> c_int {
    let s = state::get();
    if !s.use_jemalloc {
        return libc::posix_memalign(memptr, alignment, size);
    }
    if alignment < std::mem::size_of::<*mut c_void>() || !alignment.is_power_of_two() {
        *memptr = std::ptr::null_mut();
        return libc::EINVAL;
    }
    let old_errno = platform::errno();
    *memptr = je::mallocx(size, jemalloc::mallocx_align_flags(s.arena_index, alignment));
    if (*memptr).is_null() {
        let ret = platform::errno();
        platform::set_errno(old_errno);
        return if ret == 0 { libc::ENOMEM } else { ret };
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn hmmap(
    addr: *mut c_void,
    length: size_t,
    prot: c_int,
    flags: c_int,
    fd: c_int,
    offset: off_t,
) -> *mut c_void {
    alloc::hmmap_raw(addr, length, prot, flags, fd, offset)
}

#[no_mangle]
pub unsafe extern "C" fn hmunmap(addr: *mut c_void, length: size_t) -> c_int {
    libc::munmap(addr, length)
}

#[no_mangle]
pub unsafe extern "C" fn hmalloc_usable_size(ptr: *mut c_void) -> size_t {
    let s = state::get();
    if !s.use_jemalloc {
        return platform::system_malloc_usable_size(ptr);
    }
    if ptr.is_null() {
        return 0;
    }
    je::sallocx(ptr, 0)
}
