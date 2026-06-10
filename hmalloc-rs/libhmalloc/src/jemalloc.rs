use libc::{c_int, c_uint, c_void, size_t};
use tikv_jemalloc_sys as je;

// Mirrors C: MALLOCX_ARENA(arena_index) | MALLOCX_TCACHE_NONE
// Uses the crate-provided macros so flag encoding always matches the
// linked jemalloc (avoids hand-rolling the bit layout).
pub fn mallocx_flags(arena: u32) -> c_int {
    je::MALLOCX_ARENA(arena as usize) | je::MALLOCX_TCACHE_NONE
}

// Mirrors C: MALLOCX_ALIGN(alignment) | MALLOCX_ARENA(...) | MALLOCX_TCACHE_NONE
pub fn mallocx_align_flags(arena: u32, alignment: size_t) -> c_int {
    je::MALLOCX_ALIGN(alignment) | mallocx_flags(arena)
}

unsafe extern "C" fn extent_alloc(
    _hooks: *mut je::extent_hooks_t,
    _new_addr: *mut c_void,
    size: size_t,
    _alignment: size_t,
    _zero: *mut bool,
    _commit: *mut bool,
    _arena_ind: c_uint,
) -> *mut c_void {
    // Matches C extent_alloc: hmmap(NULL, size, RW, PRIVATE|ANON, 0, 0)
    crate::alloc::hmmap_raw(
        std::ptr::null_mut(),
        size,
        libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_PRIVATE | libc::MAP_ANON,
        0,
        0,
    )
}

unsafe extern "C" fn extent_dalloc(
    _hooks: *mut je::extent_hooks_t,
    addr: *mut c_void,
    size: size_t,
    _committed: bool,
    _arena_ind: c_uint,
) -> bool {
    libc::munmap(addr, size) != 0
}

// jemalloc holds a pointer to this for the arena's lifetime — must be static
static mut EXTENT_HOOKS: je::extent_hooks_t = je::extent_hooks_t {
    alloc: Some(extent_alloc),
    dalloc: Some(extent_dalloc),
    destroy: None,
    commit: None,
    decommit: None,
    purge_lazy: None,
    purge_forced: None,
    split: None,
    merge: None,
};

pub unsafe fn create_arena() -> u32 {
    let dummy = je::mallocx(1, 0);
    if !dummy.is_null() {
        je::dallocx(dummy, 0);
    }

    let mut arena_index: c_uint = 0;
    let mut unsigned_size = std::mem::size_of::<c_uint>();

    // Create arena without hooks first
    let err = je::mallctl(
        c"arenas.create".as_ptr(),
        &mut arena_index as *mut _ as *mut c_void,
        &mut unsigned_size,
        std::ptr::null_mut(),
        0,
    );
    assert_eq!(err, 0, "jemalloc arenas.create failed: {}", err);

    // Now set the extent hooks for the created arena
    let hooks_ptr: *mut je::extent_hooks_t = std::ptr::addr_of_mut!(EXTENT_HOOKS);
    let name = std::ffi::CString::new(format!("arena.{}.extent_hooks", arena_index)).unwrap();

    let err_hooks = je::mallctl(
        name.as_ptr(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        &hooks_ptr as *const _ as *mut c_void,
        std::mem::size_of::<*mut je::extent_hooks_t>(),
    );
    assert_eq!(
        err_hooks, 0,
        "jemalloc setting extent_hooks failed: {}",
        err_hooks
    );

    arena_index
}
