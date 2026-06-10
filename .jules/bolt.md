## 2024-06-25 - Use MALLOCX_ZERO for jemalloc initialization
**Learning:** `jemalloc` offers a `MALLOCX_ZERO` flag to zero out the memory allocated using `mallocx` efficiently, which could avoid page fault mapping OS pages compared to performing manual `memset`.
**Action:** When working on memory allocation optimizations, investigate the underlying library options rather than relying on manual manipulation (`libc::memset`).
