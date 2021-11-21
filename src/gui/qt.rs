use qt_widgets::QApplication;

pub fn start() -> ! {
    QApplication::init(|_app| unsafe { QApplication::exec() })
}
