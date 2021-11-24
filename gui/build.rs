#[cfg(feature = "gui_sixtyfps")]
fn main() -> Result<(), sixtyfps_build::CompileError> {
    println!("cargo:rerun-if-changed=ui/sixtyfps/main.60");
    sixtyfps_build::compile("ui/sixtyfps/main.60")
}

#[cfg(not(feature = "gui_sixtyfps"))]
fn main() {}
