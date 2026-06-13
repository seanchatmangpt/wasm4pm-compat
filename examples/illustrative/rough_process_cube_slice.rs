#![feature(adt_const_params, unsized_const_params)]
#![allow(incomplete_features)]

//! A "rough" Process Cube slicing engine implementation.
//!
//! Goal: Demonstrate how `ProcessCube`, `CubeDimension`, and `CubeSlice` shapes
//! organize sub-log partitions.
//!
//! Run with: `cargo run --example rough_process_cube_slice`

use std::marker::PhantomData;
use wasm4pm_compat::eventlog::{Event, EventLog, Trace};
use wasm4pm_compat::process_cube::{CubeDimension, CubeSlice, ProcessCube};

/// Slices an event log along a specific dimension and value.
///
/// ## Rough Implementation
///
/// In a real system, this graduates to the `wasm4pm` engine which handles
/// efficient partitioning, model discovery per cell, and conformance.
/// This implementation shows the structural relationship between the log
/// and the cube shapes.
fn slice<const DIM_NAME: &'static str>(
    _dimension: CubeDimension<DIM_NAME>,
    log: &EventLog,
    value: &str,
) -> EventLog {
    let mut filtered_traces = Vec::new();

    for trace in log.traces() {
        // In this rough version, we filter events within each trace.
        // A slice might also filter entire traces based on case attributes.
        let filtered_events: Vec<_> = trace
            .events()
            .iter()
            .filter(|e| match DIM_NAME {
                "resource" => e.resource() == Some(value),
                "activity" => e.activity() == value,
                _ => false, // Rough implementation only supports resource and activity
            })
            .cloned()
            .collect();

        if !filtered_events.is_empty() {
            // We preserve the case ID but only keep the events that match the slice.
            filtered_traces.push(Trace::new(trace.case_id(), filtered_events));
        }
    }

    EventLog::from_traces(filtered_traces)
}

fn main() {
    println!("== wasm4pm-compat: Rough Process Cube Slicing ==");

    // 1. Setup a basic event log with multiple resources and activities
    let log = EventLog::from_traces([
        Trace::new(
            "case-1",
            [
                Event::new("order").by("alice"),
                Event::new("pay").by("bob"),
                Event::new("ship").by("alice"),
            ],
        ),
        Trace::new(
            "case-2",
            [
                Event::new("order").by("alice"),
                Event::new("cancel").by("alice"),
            ],
        ),
        Trace::new(
            "case-3",
            [Event::new("order").by("bob"), Event::new("pay").by("bob")],
        ),
    ]);

    println!(
        "Original log: {} traces, {} events",
        log.trace_count(),
        log.event_count()
    );

    // 2. Perform slicing
    // We use the dimension markers to drive the slicing logic.
    // CubeDimension is a zero-cost marker; we can just pass it by value.
    // Since it doesn't implement Copy, we just use it directly in the calls.

    println!("\nSlicing by resource='alice'...");
    let alice_log = slice(CubeDimension::<"resource">, &log, "alice");
    println!(
        "Alice sub-log: {} traces, {} events",
        alice_log.trace_count(),
        alice_log.event_count()
    );

    println!("\nSlicing by activity='pay'...");
    let pay_log = slice(CubeDimension::<"activity">, &log, "pay");
    println!(
        "Pay sub-log: {} traces, {} events",
        pay_log.trace_count(),
        pay_log.event_count()
    );

    // 3. Organize using ProcessCube and CubeSlice shapes
    // A ProcessCube declared over 2 dimensions (resource and activity).
    let _cube: ProcessCube<EventLog, 2> = ProcessCube::new();

    // A CubeSlice represents the binding of a dimension to a value.
    let alice_slice: CubeSlice<CubeDimension<"resource">, &str> = CubeSlice {
        dimension: PhantomData,
        value: "alice",
    };

    println!("\nOrganized Slice metadata:");
    println!("  Dimension : resource");
    println!("  Value     : {}", alice_slice.value);
    println!("  Sub-log   : {} events", alice_log.event_count());

    // 4. Demonstrate "rough" cell intersection (resource='alice' AND activity='order')
    println!("\nIntersecting resource='alice' and activity='order'...");
    let order_log = slice(CubeDimension::<"activity">, &log, "order");
    let alice_order_log = slice(CubeDimension::<"resource">, &order_log, "alice");

    println!(
        "Cell (alice, order): {} traces, {} events",
        alice_order_log.trace_count(),
        alice_order_log.event_count()
    );

    println!("\n(Slicing computation graduates to wasm4pm; these shapes organize the results)");
}
