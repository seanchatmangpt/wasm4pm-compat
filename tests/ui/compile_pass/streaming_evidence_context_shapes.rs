#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]
// Law: StreamingContextLaw — OnlineMonitoringContext and OfflineAnalysisContext are distinct
// types; ContextualEvidence<T, Online> and ContextualEvidence<T, Offline> cannot be confused.

// COMPILE-PASS: Streaming evidence context shapes compile and the online/offline
// distinction is enforced at the type level.
//
// Law: StreamingContextLaw — ContextualEvidence<T, OnlineMonitoringContext> and
// ContextualEvidence<T, OfflineAnalysisContext> are different types. EventWindow<T, SIZE>
// carries a compile-time size constant. OnlineEvidence<T> and OfflineEvidence<T> are
// type aliases that resolve correctly.

use wasm4pm_compat::streaming::{
    ContextualEvidence,
    EventWindow,
    OfflineAnalysisContext,
    OfflineEvidence,
    OnlineEvidence,
    OnlineMonitoringContext,
    StreamingSource,
};

fn check_streaming_source() {
    // StreamingSource<N> is a zero-cost marker.
    let _: StreamingSource<64> = StreamingSource;
    let _: StreamingSource<256> = StreamingSource;
}

fn check_event_window() {
    // EventWindow<T, SIZE> constructs and has a working default.
    let w: EventWindow<u8, 128> = EventWindow::new();
    let _: EventWindow<u8, 128> = EventWindow::default();
    drop(w);

    let _: EventWindow<u32, 1> = EventWindow::new();
    let _: EventWindow<u32, 1024> = EventWindow::new();
}

fn check_offline_evidence() {
    let ev = ContextualEvidence::<u32, OfflineAnalysisContext>::offline(42);
    assert_eq!(ev.inner, 42);

    // Type alias resolves.
    let ev2: OfflineEvidence<u32> = ContextualEvidence::offline(99);
    assert_eq!(ev2.inner, 99);
}

fn check_online_evidence() {
    let ev = ContextualEvidence::<u32, OnlineMonitoringContext>::online(7);
    assert_eq!(ev.inner, 7);

    // Type alias resolves.
    let ev2: OnlineEvidence<u32> = ContextualEvidence::online(55);
    assert_eq!(ev2.inner, 55);
}

fn check_types_are_distinct() {
    // These must be *different types* — the function below accepts only offline.
    fn offline_only(_: OfflineEvidence<u64>) {}

    let offline: OfflineEvidence<u64> = ContextualEvidence::offline(1u64);
    offline_only(offline);

    // (Compile-fail of passing online to offline_only is tested in compile_fail.)
}

fn main() {
    check_streaming_source();
    check_event_window();
    check_offline_evidence();
    check_online_evidence();
    check_types_are_distinct();
}
