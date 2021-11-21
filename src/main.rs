#![deny(warnings, clippy::all, clippy::nursery, clippy::pedantic)]

pub mod gui;

fn main() {
    gui::start();
}
