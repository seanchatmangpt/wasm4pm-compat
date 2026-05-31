//! Construct the case-centric event-log shape and print a summary.
//!
//! Run with: `cargo run --example basic_eventlog`
//!
//! This example only *builds and validates* structure — it does not discover a
//! model or run conformance. Those graduate to `wasm4pm`.

use wasm4pm_compat::eventlog::{Event, EventLog, EventStream, Trace};

fn main() {
    // A single case: place_order -> approve -> ship, with timestamps & resources.
    let trace = Trace::new(
        "case-001",
        [
            Event::new("place_order").at_ns(1_000).by("web"),
            Event::new("approve").at_ns(2_000).by("alice"),
            Event::new("ship").at_ns(3_000).by("warehouse"),
        ],
    );

    // A second, shorter case.
    let trace2 = Trace::new(
        "case-002",
        [
            Event::new("place_order").at_ns(1_500),
            Event::new("cancel").at_ns(2_500),
        ],
    );

    let log = EventLog::from_traces([trace, trace2]);

    println!("== wasm4pm-compat: case-centric event log ==");
    println!("traces : {}", log.trace_count());
    println!("events : {}", log.event_count());

    match log.validate() {
        Ok(()) => println!("validate: OK (structurally well-shaped)"),
        Err(refusal) => println!("validate: REFUSED — {refusal}"),
    }

    for t in log.traces() {
        let activities: Vec<&str> = t.events().iter().map(Event::activity).collect();
        println!("  case {:<10} : {}", t.case_id(), activities.join(" -> "));
    }

    // The online sibling of a log: an append-only stream buffer.
    let mut stream = EventStream::new();
    stream.push(Event::new("heartbeat").at_ns(10_000));
    stream.push(Event::new("heartbeat").at_ns(20_000));
    println!("stream : {} buffered event(s)", stream.len());

    println!("(structure only — discovery & conformance graduate to wasm4pm)");
}
