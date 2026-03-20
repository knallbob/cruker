fn main() {
    println!("cargo:rustc-link-arg-bin=cruker=-Tsrc/ld/virt.ld");
}