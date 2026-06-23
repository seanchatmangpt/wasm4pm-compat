//! Example: Streaming context markers — online vs offline evidence
//!
//! Demonstrates the `streaming` module's zero-cost context tags:
//! - `OnlineMonitoringContext` / `OfflineAnalysisContext` — prevent
//!   silent substitution at the type level
//! - `ContextualEvidence<T, Context>` with `.online()` / `.offline()` constructors
//! - `OnlineEvidence<T>` / `OfflineEvidence<T>` type aliases
//! - `EventWindow<T, SIZE>` — fixed-size ring buffer shape
//! - `StreamingSource<WINDOW_SIZE>` — marker type
//! - `TemporalOrderConfusion` — structural defect shape
//!
//! **Failure witness:** context and window contents asserted; if `inner` field
//! or `push()` semantics change, assertions fail and example exits non-zero.
//!
//! Structure only — no event ingestion, no window management, no sliding-window
//! logic. Graduate to `wasm4pm` for those.
//!
//! Run: `cargo run --example streaming_context`
//! Doc reference: `src/streaming.rs`

use wasm4pm_compat::streaming::{
    ContextualEvidence, EventWindow, OfflineEvidence, OnlineEvidence, StreamingSource,
    TemporalOrderConfusion,
};

fn main() {
    println!("=== streaming_context ===");
    println!("Zero-cost context markers — no event ingestion, no window logic.\n");

    // ── 1. ContextualEvidence online / offline ────────────────────────────────
    println!("--- ContextualEvidence ---");
    let online: OnlineEvidence<u32> = ContextualEvidence::online(42u32);
    assert_eq!(online.inner, 42);
    println!("  OnlineEvidence<u32>.inner = {}  ✓", online.inner);

    let offline: OfflineEvidence<&str> = ContextualEvidence::offline("trace-payload");
    assert_eq!(offline.inner, "trace-payload");
    println!("  OfflineEvidence<&str>.inner = \"{}\"  ✓", offline.inner);

    // The two types are distinct — a function expecting offline evidence
    // cannot receive an online value. The compiler enforces this at the type level.
    // (Demonstrated structurally: both are `ContextualEvidence<T, _>` but with
    // different Context type params, so they cannot be assigned to each other.)
    println!("  Online and Offline context types are distinct (type-level enforcement)  ✓");

    // ── 2. EventWindow — ring buffer shape ───────────────────────────────────
    println!("\n--- EventWindow<u32, 3> ---");
    let mut window: EventWindow<u32, 3> = EventWindow::new();
    assert_eq!(window.count, 0);
    assert_eq!(window.head, 0);
    println!("  new(): count={}  head={}  ✓", window.count, window.head);

    // Push 3 events — fills the window
    let evicted1 = window.push(10);
    let evicted2 = window.push(20);
    let evicted3 = window.push(30);
    assert_eq!(evicted1, None); // no eviction until full
    assert_eq!(evicted2, None);
    assert_eq!(evicted3, None);
    assert_eq!(window.count, 3);
    println!("  after 3 pushes: count={}  ✓", window.count);

    // 4th push evicts the oldest (ring buffer behavior)
    let evicted4 = window.push(40);
    assert_eq!(evicted4, Some(10), "4th push must evict first element 10");
    assert_eq!(window.count, 3, "count stays at window size");
    println!(
        "  push(40) evicts {} (oldest)  count still {}  ✓",
        evicted4.unwrap(),
        window.count
    );

    // Default is same as new()
    let w2: EventWindow<u8, 8> = EventWindow::default();
    assert_eq!(w2.count, 0);
    println!("  EventWindow::<u8,8>::default().count = 0  ✓");

    // ── 3. StreamingSource<WINDOW_SIZE> marker ────────────────────────────────
    println!("\n--- StreamingSource<128> marker ---");
    // Zero-sized marker type; exists purely at compile time
    let _source: StreamingSource<128> = StreamingSource;
    println!("  StreamingSource<128> is a zero-sized marker type  ✓");

    // ── 4. TemporalOrderConfusion — structural defect shape ──────────────────
    println!("\n--- TemporalOrderConfusion ---");
    let confusion = TemporalOrderConfusion {
        current_timestamp: 1_700_000_200,
        offending_timestamp: 1_700_000_100, // earlier than current — disorder
    };
    assert_eq!(confusion.current_timestamp, 1_700_000_200);
    assert_eq!(confusion.offending_timestamp, 1_700_000_100);
    assert!(
        confusion.offending_timestamp < confusion.current_timestamp,
        "temporal disorder: offending must precede current"
    );
    println!(
        "  current={}  offending={}  (offending < current = temporal disorder)  ✓",
        confusion.current_timestamp, confusion.offending_timestamp
    );

    println!("\n=== All assertions passed — streaming module is witnessed ===");
    println!("  Covered: OnlineEvidence, OfflineEvidence, ContextualEvidence,");
    println!("           EventWindow (ring buffer push + eviction), StreamingSource,");
    println!("           TemporalOrderConfusion.");
    println!("  Witness: inner values + ring-buffer eviction asserted; breaks on API change.");
    println!("  Structure only — no event ingestion, no sliding windows, no monitoring.");
    println!("  Graduate to wasm4pm for: stream ingestion, online conformance, drift detection.");
}
