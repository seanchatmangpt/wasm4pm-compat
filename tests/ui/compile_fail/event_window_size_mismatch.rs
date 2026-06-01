// COMPILE-FAIL: EventWindow const size mismatch — different window sizes are different types.
//
// Law: EventWindowSizeLaw — EventWindow<T,10> and EventWindow<T,20> are distinct types.
// A function requiring a 20-event window cannot accept a 10-event window.
// The SIZE const parameter enforces window capacity at the type level.
use wasm4pm_compat::streaming::EventWindow;

fn requires_window_20(_window: EventWindow<u8, 20>) {}

fn main() {
    let window_10: EventWindow<u8, 10> = EventWindow::new();
    // This must fail: EventWindow<u8, 10> is not EventWindow<u8, 20>.
    // Different window sizes are non-interchangeable at the type level.
    requires_window_20(window_10);
}
