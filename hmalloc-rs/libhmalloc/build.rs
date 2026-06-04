fn main() {
    println!("cargo:rustc-link-lib=jemalloc");
    println!("cargo:rustc-link-lib=numa");
}
