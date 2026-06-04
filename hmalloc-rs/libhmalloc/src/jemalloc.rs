use libc::{c_int, c_uint, c_void, size_t};
use tikv_jemalloc_sys as je;

// MALLOCX_ARENA(a) = ((int)(a) + 1) << 20
// MALLOCX_TCACHE_NONE = (-1 << 8)
pub fn mallocx_flags(arena: u32) -> c_int {
    let arena_flag = ((arena as c_int) + 1) << 20;
    let tcache_none: c_int = -1 << 8;
    arena_flag | tcache_none
}

// MALLOCX_ALIGN(a) encodes log2(a) in bits [0..7]
pub fn mallocx_align_flags(arena: u32, alignment: size_t) -> c_int {
    let align_bits = alignment.trailing_zeros() as c_int;
    mallocx_flags(arena) | align_bits
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
    crate::alloc::hmmap_raw(
        std::ptr::null_mut(),
        size,
        libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_PRIVATE | libc::MAP_ANON,
        -1,
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
    let mut arena_index: c_uint = 0;
    let mut unsigned_size = std::mem::size_of::<c_uint>();
    let hooks_ptr: *mut je::extent_hooks_t = std::ptr::addr_of_mut!(EXTENT_HOOKS);

    let err = je::mallctl(
        b"arenas.create\0".as_ptr() as *const libc::c_char,
        &mut arena_index as *mut _ as *mut c_void,
        &mut unsigned_size,
        &hooks_ptr as *const _ as *mut c_void,
        std::mem::size_of::<*mut je::extent_hooks_t>(),
    );
    assert_eq!(err, 0, "jemalloc arenas.create failed: {}", err);
    arena_index
}
