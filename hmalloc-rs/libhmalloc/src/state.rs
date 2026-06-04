use once_cell::sync::OnceCell;
use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};

pub struct AllocState {
    pub use_jemalloc: bool,
    pub arena_index: u32,
}

static STATE: OnceCell<AllocState> = OnceCell::new();
static NODEMASK: AtomicU64 = AtomicU64::new(0);
static MPOL_MODE: AtomicI32 = AtomicI32::new(0);
static MAXNODE: AtomicI32 = AtomicI32::new(0);

pub fn get() -> &'static AllocState {
    STATE.get_or_init(init_state)
}

pub fn get_policy() -> (u64, i32, i32) {
    (
        NODEMASK.load(Ordering::Relaxed),
        MPOL_MODE.load(Ordering::Relaxed),
        MAXNODE.load(Ordering::Relaxed),
    )
}

fn init_state() -> AllocState {
    let use_jemalloc = crate::env::read_jemalloc();
    let nodemask = crate::env::read_nodemask();
    let mpol_mode = crate::env::read_mpol_mode();
    let mut arena_index = 0u32;
    let maxnode;

    NODEMASK.store(nodemask, Ordering::Relaxed);
    MPOL_MODE.store(mpol_mode, Ordering::Relaxed);

    if use_jemalloc {
        unsafe {
            maxnode = crate::numa::max_possible_node();
            MAXNODE.store(maxnode, Ordering::Relaxed);
            arena_index = crate::jemalloc::create_arena();
        }
    }

    AllocState { use_jemalloc, arena_index }
}
