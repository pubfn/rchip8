#[cfg(has_qt_cpp)]
unsafe extern "C" {
    fn chip8_qt_run() -> i32;
}

pub fn run() {
    #[cfg(has_qt_cpp)]
    {
        unsafe {
            let _ = chip8_qt_run();
        }
        return;
    }

    #[cfg(not(has_qt_cpp))]
    {
        eprintln!(
            "Qt bridge is not available: Qt5Widgets development files were not found by pkg-config."
        );
    }
}
