fn main() {
    println!("cargo:rustc-link-search=native=SDL2/lib/x64");
    println!("cargo:rustc-link-lib=SDL2");
}
