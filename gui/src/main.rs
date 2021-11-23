#![deny(warnings, clippy::all)]

pub mod consts;
pub mod gui;

fn main() {
    gui::start();
}
