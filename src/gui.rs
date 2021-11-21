#[cfg(feature = "gui_qt")]
pub mod qt;

#[cfg(feature = "gui_qt")]
pub fn start() -> ! {
    qt::start()
}
