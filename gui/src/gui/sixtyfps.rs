sixtyfps::include_modules!();

use lmms_core::engine::Engine;
use sixtyfps::{ModelHandle, VecModel};
use std::fs::File;
use std::lazy::SyncOnceCell;
use std::rc::Rc;
use std::sync::RwLock;

static ENGINE_CELL: SyncOnceCell<RwLock<Engine>> = SyncOnceCell::new();

/// # Panics
/// It doesn't panic. Don't worry.
pub fn start(engine: Engine) {
    let window = MainWindow::new();
    let tracks = VecModel::default();
    let patterns = VecModel::default();
    ENGINE_CELL.set(RwLock::new(engine)).unwrap_or(());
    patterns.push((
        TrackPattern {
            length: 1.0,
            notes: ModelHandle::default(),
        },
        0.0,
    ));
    tracks.push(Track {
        patterns: ModelHandle::new(Rc::new(patterns)),
    });
    window.set_tracks(ModelHandle::new(Rc::new(tracks)));
    window.on_open(|| {
        ENGINE_CELL.get().unwrap().write().unwrap().load_project_file(
            &mut File::open("/home/rdrpenguin/Documents-Shared/lmms/templates/default.mpt")
                .unwrap(),
        ).unwrap();
    });
    window.run();
    drop(window);
}
