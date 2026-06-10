## 2024-06-08 - Fast `calloc` via `MALLOCX_ZERO`
**Learning:** In custom allocators wrapping `jemalloc`, manually zeroing memory using `memset` destroys performance by synchronously forcing page faults and physical memory allocation.
**Action:** Always delegate zeroing back to the underlying allocator (e.g., `je::MALLOCX_ZERO` flag or `libc::calloc`) so it can leverage OS zero-pages dynamically, turning an O(N) wait into an O(1) return.
