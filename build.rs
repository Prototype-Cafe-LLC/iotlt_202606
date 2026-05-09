//! Copy `memory.x` into `OUT_DIR` so the linker always finds it.

fn main() {
    let out = std::env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search={out}");
    println!("cargo:rerun-if-changed=memory.x");
    std::fs::write(format!("{out}/memory.x"), include_bytes!("memory.x")).unwrap();
}
