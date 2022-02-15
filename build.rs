fn main() {
    #[cfg(any(target_os = "openbsd", target_os = "freebsd"))]
    println!(r"cargo:rustc-link-search=/usr/local/lib");

    if let Ok(sdl_path) = env::var("SDL2_STATIC_LIB_PATH") {
        println!(
            "cargo:rustc-link-search={}",
            sdl_path
        );
    }

}
