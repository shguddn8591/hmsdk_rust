use once_cell::sync::OnceCell;

pub struct AllocState {
    pub use_jemalloc: bool,
    pub nodemask: u64,
    pub mpol_mode: i32,
    pub arena_index: u32,
    pub maxnode: i32,
}

static STATE: OnceCell<AllocState> = OnceCell::new();

pub fn get() -> &'static AllocState {
    STATE.get_or_init(init_state)
}

fn init_state() -> AllocState {
    let use_jemalloc = crate::env::read_jemalloc();
    let nodemask = crate::env::read_nodemask();
    let mpol_mode = crate::env::read_mpol_mode();
    let mut arena_index = 0u32;
    let mut maxnode = 0i32;

    if use_jemalloc {
        unsafe {
            maxnode = crate::numa::max_possible_node();
            arena_index = crate::jemalloc::create_arena();
        }
    }

    AllocState { use_jemalloc, nodemask, mpol_mode, arena_index, maxnode }
}
