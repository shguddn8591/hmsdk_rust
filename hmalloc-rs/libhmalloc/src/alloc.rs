use libc::{c_int, c_void, off_t, size_t};

// Used by extent hook (no NUMA policy — called from jemalloc's arena allocation path)
pub unsafe fn hmmap_raw(
    addr: *mut c_void,
    length: size_t,
    prot: c_int,
    flags: c_int,
    fd: c_int,
    offset: off_t,
) -> *mut c_void {
    // Use get_policy() to read NUMA params via Atomics.
    // Calling state::get() here during arena creation would cause a deadlock
    // on the OnceCell because extent_alloc runs within init_state.
    let (nodemask, mpol_mode, maxnode) = crate::state::get_policy();
    hmmap_with_policy(addr, length, prot, flags, fd, offset, nodemask, mpol_mode, maxnode)
}

// Parameterised version used by public hmmap() and tests
#[allow(clippy::too_many_arguments)]
pub unsafe fn hmmap_with_policy(
    addr: *mut c_void,
    length: size_t,
    prot: c_int,
    flags: c_int,
    fd: c_int,
    offset: off_t,
    nodemask: u64,
    mpol_mode: i32,
    maxnode: i32,
) -> *mut c_void {
    let new_addr = libc::mmap(addr, length, prot, flags, fd, offset);
    if new_addr == libc::MAP_FAILED {
        return libc::MAP_FAILED;
    }

    if nodemask > 0 {
        // Matches C: mbind(new_addr, length, mpol_mode, &nodemask, maxnode, 0)
        let ret = crate::numa::mbind(
            new_addr,
            length,
            mpol_mode,
            &nodemask as *const u64 as *const libc::c_ulong,
            maxnode as libc::c_ulong,
            0,
        );
        if ret != 0 {
            let saved_errno = crate::platform::errno();
            libc::munmap(new_addr, length);
            crate::platform::set_errno(saved_errno);
            return std::ptr::null_mut();
        }
    }

    new_addr
}
