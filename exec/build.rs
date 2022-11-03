fn main() {
    println!("cargo:rustc-link-search=all=.");
    println!("cargo:rustc-link-lib=dylib=magic.o");
}
