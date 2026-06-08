use criterion::{criterion_group, criterion_main, Criterion};

#[link(name = "c")]
extern "C" {
    fn calloc(nmemb: libc::size_t, size: libc::size_t) -> *mut libc::c_void;
    fn free(ptr: *mut libc::c_void);
}

fn bench_calloc(c: &mut Criterion) {
    let mut group = c.benchmark_group("calloc");
    group.sample_size(10);
    group.bench_function("libc_calloc", |b| {
        b.iter(|| unsafe {
            let ptr = calloc(1, 1024 * 1024 * 128); // 128MB
            std::hint::black_box(ptr);
            free(ptr);
        });
    });
    group.bench_function("hcalloc_zeroed", |b| {
        b.iter(|| unsafe {
            std::env::set_var("HMALLOC_JEMALLOC", "1");
            let ptr = hmalloc::hcalloc(1, 1024 * 1024 * 128); // 128MB
            std::hint::black_box(ptr);
            hmalloc::hfree(ptr);
        });
    });
    group.bench_function("hcalloc_MALLOCX_ZERO", |b| {
        b.iter(|| unsafe {
            std::env::set_var("HMALLOC_JEMALLOC", "1");
            // Simulate MALLOCX_ZERO
            let arena = 0; // assuming arena 0
            let ptr = tikv_jemalloc_sys::mallocx(
                1024 * 1024 * 128,
                tikv_jemalloc_sys::MALLOCX_ARENA(arena)
                    | tikv_jemalloc_sys::MALLOCX_TCACHE_NONE
                    | tikv_jemalloc_sys::MALLOCX_ZERO,
            );
            std::hint::black_box(ptr);
            tikv_jemalloc_sys::dallocx(
                ptr,
                tikv_jemalloc_sys::MALLOCX_ARENA(arena) | tikv_jemalloc_sys::MALLOCX_TCACHE_NONE,
            );
        });
    });
    group.finish();
}

criterion_group!(benches, bench_calloc);
criterion_main!(benches);
