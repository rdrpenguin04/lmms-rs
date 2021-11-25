#[cfg(feature = "gui_sixtyfps")]
pub mod sixtyfps;

use lmms_core::engine::Engine;

#[cfg(feature = "gui_sixtyfps")]
pub fn start(engine: Engine) {
    sixtyfps::start(engine);
}
