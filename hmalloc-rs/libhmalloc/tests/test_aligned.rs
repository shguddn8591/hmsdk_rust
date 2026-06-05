use hmalloc::{haligned_alloc, hfree, hposix_memalign};
use libc::c_void;

fn is_aligned(ptr: *mut c_void, alignment: usize) -> bool {
    (ptr as usize).is_multiple_of(alignment)
}

#[test]
fn haligned_alloc_basic() {
    unsafe {
        for &align in &[16usize, 64, 4096] {
            let ptr = haligned_alloc(align, align * 4);
            assert!(!ptr.is_null(), "align={}", align);
            assert!(is_aligned(ptr, align), "ptr not aligned to {}", align);
            hfree(ptr);
        }
    }
}

#[test]
fn hposix_memalign_basic() {
    unsafe {
        for &align in &[8usize, 64, 512, 4096] {
            let mut ptr: *mut c_void = std::ptr::null_mut();
            let ret = hposix_memalign(&mut ptr, align, 1024);
            assert_eq!(ret, 0, "align={}", align);
            assert!(!ptr.is_null());
            assert!(is_aligned(ptr, align));
            hfree(ptr);
        }
    }
}

#[test]
fn hposix_memalign_invalid_alignment() {
    unsafe {
        let mut ptr: *mut c_void = std::ptr::null_mut();
        let ret = hposix_memalign(&mut ptr, 7, 64); // 7 is not a power of 2
        assert_eq!(ret, libc::EINVAL);
    }
}
