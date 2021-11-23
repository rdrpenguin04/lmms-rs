#[cfg(feature = "gui_sixtyfps")]
fn main() -> Result<(), sixtyfps_build::CompileError> {
    sixtyfps_build::compile("ui/sixtyfps/main.60")
}

#[cfg(not(feature = "gui_sixtyfps"))]
fn main() {}
