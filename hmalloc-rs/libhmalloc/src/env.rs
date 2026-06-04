use std::env;

pub fn read_jemalloc() -> bool {
    env::var("HMALLOC_JEMALLOC").map(|v| v == "1").unwrap_or(false)
}

pub fn read_nodemask() -> u64 {
    env::var("HMALLOC_NODEMASK")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}

pub fn read_mpol_mode() -> i32 {
    env::var("HMALLOC_MPOL_MODE")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}
