#[cfg(feature = "gui_sixtyfps")]
pub mod sixtyfps;

#[cfg(feature = "gui_sixtyfps")]
pub fn start() {
    sixtyfps::start();
}
