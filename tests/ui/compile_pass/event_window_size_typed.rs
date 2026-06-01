// COMPILE-PASS: EventWindow const size typed — windows of different sizes are distinct types
// and each is accepted by its size-specific function.
//
// Law: EventWindowSizeLaw — the SIZE const parameter enforces window capacity at the type level.
use wasm4pm_compat::streaming::{EventWindow, StreamingSource};

fn requires_window_10(_w: EventWindow<u8, 10>) {}
fn requires_window_20(_w: EventWindow<u8, 20>) {}

fn main() {
    let w10: EventWindow<u8, 10> = EventWindow::new();
    let w20: EventWindow<u8, 20> = EventWindow::new();

    requires_window_10(w10);
    requires_window_20(w20);

    // Default construction works
    let _w128: EventWindow<u8, 128> = EventWindow::default();

    // StreamingSource is a zero-sized const-generic marker
    let _src: StreamingSource<256> = StreamingSource;
}
