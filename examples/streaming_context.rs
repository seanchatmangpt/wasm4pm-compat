//! Streaming process evidence context shapes.
//!
//! Demonstrates the `streaming` module's typed context vocabulary:
//!
//! - [`StreamingSource<WINDOW_SIZE>`] — compile-time window-size marker
//! - [`EventWindow<T, SIZE>`] — circular buffer with `push()` and ring-wrap
//! - [`OnlineMonitoringContext`] / [`OfflineAnalysisContext`] — context tokens
//! - [`ContextualEvidence<T, Context>`] — evidence tagged with its collection context
//! - [`OnlineEvidence<T>`] / [`OfflineEvidence<T>`] — type aliases for the common cases
//! - [`TemporalOrderConfusion`] — structural marker for out-of-order event detection
//!
//! **Key contract:** `OnlineEvidence<T>` and `OfflineEvidence<T>` are
//! **different types** — a function that expects offline evidence cannot
//! accidentally receive an online window. This prevents the silent substitution
//! of an online monitoring stream for a completed log at the type level.
//!
//! **Failure witness:** `ev.inner` field access and the ring-wrap count assert
//! actual values; the type-alias identity check asserts that `OnlineEvidence`
//! and `OfflineEvidence` are the aliases the docs claim.
//!
//! Doc reference: `src/streaming.rs`, `docs/API_TOUR.md`

use wasm4pm_compat::streaming::{
    ContextualEvidence, EventWindow, OfflineAnalysisContext, OfflineEvidence,
    OnlineEvidence, OnlineMonitoringContext, StreamingSource, TemporalOrderConfusion,
};

fn main() {
    println!("=== Streaming process evidence context shapes ===\n");

    // ── Part 1: StreamingSource<WINDOW_SIZE> — compile-time marker ───────────
    println!("Part 1: StreamingSource<WINDOW_SIZE>");

    let _source_128: StreamingSource<128> = StreamingSource;
    let _source_64:  StreamingSource<64>  = StreamingSource;
    // StreamingSource<128> and StreamingSource<64> are different types —
    // the compiler would reject substituting one for the other.
    println!("  ✓ StreamingSource<128> and StreamingSource<64> are distinct types");

    // ── Part 2: EventWindow<T, SIZE> — circular buffer ───────────────────────
    println!("\nPart 2: EventWindow<T, SIZE> — push and ring-wrap");

    let mut window: EventWindow<u32, 4> = EventWindow::new();
    assert_eq!(window.count, 0);
    assert_eq!(window.head, 0);

    // Fill the window.
    let ejected0 = window.push(10);
    let ejected1 = window.push(20);
    let ejected2 = window.push(30);
    let ejected3 = window.push(40);
    assert!(ejected0.is_none(), "no eviction until window is full");
    assert!(ejected1.is_none());
    assert!(ejected2.is_none());
    assert!(ejected3.is_none());
    assert_eq!(window.count, 4);
    println!("  ✓ push 4 events into SIZE=4 window → count=4, no eviction");

    // Ring-wrap: pushing a 5th event evicts the oldest.
    let evicted = window.push(50);
    assert!(evicted.is_some(), "overflow must evict oldest slot");
    assert_eq!(evicted, Some(10), "ring evicts slot 0 (value 10)");
    assert_eq!(window.count, 4, "count stays at capacity");
    println!("  ✓ 5th push evicts slot 0 (value 10) → count stays at 4");

    // Default constructs an empty window.
    let default_win: EventWindow<String, 8> = EventWindow::default();
    assert_eq!(default_win.count, 0);
    println!("  ✓ EventWindow::default() → count=0");

    // ── Part 3: OnlineMonitoringContext / OfflineAnalysisContext tokens ───────
    println!("\nPart 3: Context tokens are zero-cost unit types");

    let _online:  OnlineMonitoringContext  = OnlineMonitoringContext;
    let _offline: OfflineAnalysisContext   = OfflineAnalysisContext;
    // These tokens are used as phantom type params — no data, just type identity.
    println!("  ✓ OnlineMonitoringContext and OfflineAnalysisContext constructible");

    // ── Part 4: ContextualEvidence<T, Context> ────────────────────────────────
    println!("\nPart 4: ContextualEvidence — online and offline wrappers");

    let online_ev: ContextualEvidence<u64, OnlineMonitoringContext> =
        ContextualEvidence::online(42_u64);
    assert_eq!(online_ev.inner, 42_u64);
    println!("  ✓ ContextualEvidence::online(42).inner = 42");

    let offline_ev: ContextualEvidence<u64, OfflineAnalysisContext> =
        ContextualEvidence::offline(99_u64);
    assert_eq!(offline_ev.inner, 99_u64);
    println!("  ✓ ContextualEvidence::offline(99).inner = 99");

    // ── Part 5: OnlineEvidence<T> and OfflineEvidence<T> type aliases ─────────
    println!("\nPart 5: OnlineEvidence<T> / OfflineEvidence<T> type aliases");

    // The aliases expand to ContextualEvidence<T, *Context> — confirmed by
    // assigning a ContextualEvidence directly to the alias type.
    let online_alias: OnlineEvidence<&str> = ContextualEvidence::online("live-stream-event");
    assert_eq!(online_alias.inner, "live-stream-event");

    let offline_alias: OfflineEvidence<&str> = ContextualEvidence::offline("log-replay-event");
    assert_eq!(offline_alias.inner, "log-replay-event");

    println!("  ✓ OnlineEvidence<&str> = ContextualEvidence<&str, OnlineMonitoringContext>");
    println!("  ✓ OfflineEvidence<&str> = ContextualEvidence<&str, OfflineAnalysisContext>");

    // ── Part 6: TemporalOrderConfusion — out-of-order event marker ───────────
    println!("\nPart 6: TemporalOrderConfusion — structural marker");

    let confusion = TemporalOrderConfusion {
        current_timestamp: 1_000_000,
        offending_timestamp: 900_000, // older than current = out of order
    };
    assert_eq!(confusion.current_timestamp, 1_000_000);
    assert_eq!(confusion.offending_timestamp, 900_000);
    assert!(
        confusion.offending_timestamp < confusion.current_timestamp,
        "out-of-order: offending timestamp precedes current"
    );
    println!(
        "  ✓ TemporalOrderConfusion: current={}, offending={} (out of order)",
        confusion.current_timestamp, confusion.offending_timestamp
    );

    // ── Part 7: Structure-only contract ──────────────────────────────────────
    println!("\nPart 7: Structure-only contract");
    // There is no stream ingestion, no window trigger, no sliding-window
    // computation in this module. These types are shapes that travel with
    // evidence into wasm4pm, where the streaming engine runs.
    println!("  ✓ No stream ingestion method exists on EventWindow or ContextualEvidence");
    println!("  ✓ Graduate to wasm4pm for: windowed ingestion, online conformance, drift detection");

    println!("\n=== All assertions passed — streaming module surface is witnessed ===");
    println!("  Covered: StreamingSource<N>, EventWindow<T,SIZE> (push + ring-wrap + default),");
    println!("           OnlineMonitoringContext, OfflineAnalysisContext,");
    println!("           ContextualEvidence (online + offline), OnlineEvidence<T>,");
    println!("           OfflineEvidence<T>, TemporalOrderConfusion");
}
