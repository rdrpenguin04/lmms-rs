#![deny(warnings, clippy::all, clippy::pedantic)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::default_trait_access,
    clippy::erasing_op,
    clippy::float_cmp,
    clippy::if_not_else,
    clippy::items_after_statements,
    clippy::match_same_arms,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::no_effect_underscore_binding,
    clippy::redundant_clone,
    clippy::semicolon_if_nothing_returned,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::unreadable_literal,
    clippy::used_underscore_binding,
    clippy::wildcard_imports
)] // Because SixtyFPS
#![feature(once_cell)]

pub mod consts;
pub mod gui;

use lmms_core::engine::Engine;

fn main() {
    let engine = Engine::default();
    gui::start(engine);
}
