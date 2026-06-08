use std::env;

pub fn read_jemalloc() -> bool {
    env::var("HMALLOC_JEMALLOC")
        .map(|v| v == "1")
        .unwrap_or(false)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;

    // Mutex to serialize environment variable tests to prevent race conditions
    // since std::env::set_var is not thread-safe if other threads read/write it.
    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_read_jemalloc_edge_cases() {
        let _lock = ENV_MUTEX.lock().unwrap();

        env::remove_var("HMALLOC_JEMALLOC");
        assert_eq!(read_jemalloc(), false);

        env::set_var("HMALLOC_JEMALLOC", "1");
        assert_eq!(read_jemalloc(), true);

        // Invalid values
        env::set_var("HMALLOC_JEMALLOC", "2");
        assert_eq!(read_jemalloc(), false);
        env::set_var("HMALLOC_JEMALLOC", "true");
        assert_eq!(read_jemalloc(), false);
        env::set_var("HMALLOC_JEMALLOC", "");
        assert_eq!(read_jemalloc(), false);
    }

    #[test]
    fn test_read_nodemask_edge_cases() {
        let _lock = ENV_MUTEX.lock().unwrap();

        env::remove_var("HMALLOC_NODEMASK");
        assert_eq!(read_nodemask(), 0);

        env::set_var("HMALLOC_NODEMASK", "123");
        assert_eq!(read_nodemask(), 123);

        // Invalid values (should default to 0 safely without panic)
        env::set_var("HMALLOC_NODEMASK", "abc");
        assert_eq!(read_nodemask(), 0);
        env::set_var("HMALLOC_NODEMASK", "-1"); // Invalid u64
        assert_eq!(read_nodemask(), 0);
        env::set_var("HMALLOC_NODEMASK", "18446744073709551616"); // Overflow u64
        assert_eq!(read_nodemask(), 0);
    }

    #[test]
    fn test_read_mpol_mode_edge_cases() {
        let _lock = ENV_MUTEX.lock().unwrap();

        env::remove_var("HMALLOC_MPOL_MODE");
        assert_eq!(read_mpol_mode(), 0);

        env::set_var("HMALLOC_MPOL_MODE", "2");
        assert_eq!(read_mpol_mode(), 2);
        env::set_var("HMALLOC_MPOL_MODE", "-1");
        assert_eq!(read_mpol_mode(), -1);

        // Invalid values
        env::set_var("HMALLOC_MPOL_MODE", "abc");
        assert_eq!(read_mpol_mode(), 0);
        env::set_var("HMALLOC_MPOL_MODE", "2147483648"); // Overflow i32
        assert_eq!(read_mpol_mode(), 0);
    }
}
