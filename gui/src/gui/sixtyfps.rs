sixtyfps::include_modules!();

use sixtyfps::{ModelHandle, VecModel};
use std::rc::Rc;

pub fn start() {
    let window = MainWindow::new();
    let tracks = VecModel::default();
    let patterns = VecModel::default();
    patterns.push((TrackPattern {
        length: 1.0,
        notes: ModelHandle::default(),
    }, 0.0));
    tracks.push(Track {
        patterns: ModelHandle::new(Rc::new(patterns)),
    });
    window.set_tracks(ModelHandle::new(Rc::new(tracks)));
    window.run();
}
