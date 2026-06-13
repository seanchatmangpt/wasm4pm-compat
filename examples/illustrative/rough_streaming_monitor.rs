//! # Rough Streaming Process Monitor
//!
//! A simulation of a sliding window monitor that detects out-of-order events
//! using `TemporalOrder` at the window boundary.
//!
//! Run with: `cargo run --example rough_streaming_monitor`

use wasm4pm_compat::eventlog::Event;
use wasm4pm_compat::streaming::{EventWindow, OnlineEvidence};
use wasm4pm_compat::temporal::TemporalOrder;

fn main() {
    // 1. Setup a sliding window buffer for the last 4 events.
    const WINDOW_SIZE: usize = 4;
    let mut window: EventWindow<Event, WINDOW_SIZE> = EventWindow::new();

    // 2. Simulate a stream of events with some temporal noise.
    // The timestamps are in nanoseconds.
    let event_stream = vec![
        Event::new("order_created").at_ns(1000),
        Event::new("payment_received").at_ns(1100),
        Event::new("fraud_check_started").at_ns(1050), // Temporal Anomaly! (Arrived late or clock drift)
        Event::new("fraud_check_passed").at_ns(1200),
        Event::new("order_shipped").at_ns(1300),
        Event::new("order_delivered").at_ns(1250), // Another Anomaly!
    ];

    println!("=== wasm4pm-compat: Rough Streaming Monitor ===");
    println!("Window Size: {}", WINDOW_SIZE);
    println!();

    for incoming in event_stream {
        let incoming_ts = incoming.timestamp_ns().unwrap_or(0);
        print!(
            "Ingesting: {:<20} (ts: {:>5}ns) -> ",
            incoming.activity(),
            incoming_ts
        );

        // 3. Use TemporalOrder to detect out-of-order at the boundary.
        // We compare the incoming event with the most recent event in the buffer.
        if let Some(last_event) = get_last_event(&window) {
            let last_ts = last_event.timestamp_ns().unwrap_or(0);

            // Determine temporal relationship
            // If incoming_ts < last_ts, then 'incoming' happened BEFORE 'last_event'
            // but arrived AFTER. This is an out-of-order anomaly.
            let order = if incoming_ts < last_ts {
                TemporalOrder::After // The PREVIOUS event is AFTER the incoming one in time
            } else if incoming_ts > last_ts {
                TemporalOrder::Before // The PREVIOUS event is BEFORE the incoming one in time
            } else {
                TemporalOrder::Concurrent
            };

            match order {
                TemporalOrder::After => {
                    println!("\x1b[31m[OUT-OF-ORDER]\x1b[0m prev ts: {}ns", last_ts);
                }
                TemporalOrder::Before => {
                    println!("\x1b[32m[OK]\x1b[0m");
                }
                TemporalOrder::Concurrent => {
                    println!("\x1b[33m[CONCURRENT]\x1b[0m");
                }
                TemporalOrder::Unknown => {
                    println!("[UNKNOWN]");
                }
            }
        } else {
            println!("\x1b[32m[OK] (First Event)\x1b[0m");
        }

        // Push to sliding window
        window.push(incoming);
    }

    println!();
    // 4. Wrap the resulting window in OnlineEvidence to witness the context.
    let evidence: OnlineEvidence<EventWindow<Event, WINDOW_SIZE>> = OnlineEvidence::online(window);

    println!("Monitor complete. Evidence captured with OnlineMonitoringContext.");
    println!("Final window element count: {}", evidence.inner.count);
}

/// Helper to peek at the most recently pushed event in the circular buffer.
fn get_last_event<const N: usize>(window: &EventWindow<Event, N>) -> Option<&Event> {
    if window.count == 0 {
        return None;
    }
    // head points to the NEXT slot to be written.
    // So (head + N - 1) % N is the slot that was JUST written.
    let last_idx = (window.head + N - 1) % N;
    window.buffer[last_idx].as_ref()
}
