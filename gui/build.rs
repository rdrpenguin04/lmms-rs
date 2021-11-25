#[cfg(feature = "gui_sixtyfps")]
fn main() -> Result<(), sixtyfps_build::CompileError> {
    println!("cargo:rerun-if-changed=ui/sixtyfps/main.60");
    sixtyfps_build::compile_with_config(
        "ui/sixtyfps/main.60",
        sixtyfps_build::CompilerConfiguration::new().with_style(String::from("fluent")),
    )
}

#[cfg(not(feature = "gui_sixtyfps"))]
fn main() {}
