use libc::{c_int, c_long, c_uint, c_ulong, c_void, size_t};

extern "C" {
    pub fn numa_max_possible_node() -> c_int;
    pub fn mbind(
        addr: *mut c_void,
        len: size_t,
        mode: c_int,
        nodemask: *const c_ulong,
        maxnode: c_ulong,
        flags: c_uint,
    ) -> c_long;
}

pub unsafe fn max_possible_node() -> i32 {
    numa_max_possible_node()
}
