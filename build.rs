fn main() {
    println!("cargo:rustc-link-lib=dylib=seek_model");
    println!("cargo:rustc-link-search=native=project_hw/lib");
}
