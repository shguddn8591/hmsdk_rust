use std::process::Command;

#[test]
fn test_hmctl_e2e_preload() {
    let hmctl_path = env!("CARGO_BIN_EXE_hmctl");
    // We can assume the library is built in the same target/debug directory
    let lib_path = std::path::Path::new(hmctl_path)
        .parent()
        .unwrap()
        .join("libhmalloc.so");
    let lib_path_str = lib_path.to_str().unwrap();
    
    // We execute `hmctl -m 0 -- env` to see if LD_PRELOAD is set correctly
    let output = Command::new(hmctl_path)
        .arg("-m")
        .arg("0")
        .arg("--")
        .arg("env")
        .env("HMALLOC_LIB_PATH", lib_path_str)
        .output()
        .expect("Failed to execute hmctl");

    assert!(output.status.success(), "hmctl execution failed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Check if our environment variables were correctly propagated
    assert!(stdout.contains("HMALLOC_JEMALLOC=1"));
    assert!(stdout.contains("HMALLOC_MPOL_MODE=2")); // MPOL_BIND is 2
    assert!(stdout.contains("HMALLOC_NODEMASK=1")); // nodemask for 0 is 1

    // Check if LD_PRELOAD was injected
    assert!(stdout.contains(&format!("LD_PRELOAD={}", lib_path_str)));
}
