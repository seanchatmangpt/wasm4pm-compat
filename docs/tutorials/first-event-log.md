# Tutorial: Constructing Your First Case-Centric Event Log

This tutorial guides you step-by-step through building, validating, and inspecting your first case-centric event log using `wasm4pm-compat` version `26.6.23`. 

## Learning Objectives

By the end of this tutorial, you will:
1. Understand the core structural elements of case-centric logs in `wasm4pm-compat`.
2. Construct in-memory events, traces, and an event log.
3. Validate the structural integrity of your constructed log.
4. Set up an append-only event stream buffer.

---

## Prerequisites

Before starting, ensure your system is configured as follows:
- You are using **nightly Rust** (as required by the crate's type-state bounds).
- You have added `wasm4pm-compat` version `26.6.23` to your dependencies, with default features enabled.

---

## Step 1: Create a Basic Project

Initialize a new binary application and add `wasm4pm-compat` to your dependencies:

```bash
cargo new my_first_log --bin
cd my_first_log
```

In your `Cargo.toml`, specify the dependency:

```toml
[dependencies]
wasm4pm-compat = { version = "26.6.23" }
```

---

## Step 2: Import Core Types

Open `src/main.rs` and import the necessary modules. We will use the base event-log types:

```rust
use wasm4pm_compat::eventlog::{Event, Trace, EventLog, EventStream};
```

---

## Step 3: Instantiate Event Structures

A case-centric event represents an activity execution. It requires an activity name, and can builder-style attach a timestamp and originating resource.

Let's construct two events:

```rust
fn main() {
    // 1. Create a "Register Order" event
    let event_1 = Event::new("Register Order")
        .at_ns(1_700_000_000_000_000_000)
        .by("SystemAgent");

    // 2. Create a "Confirm Payment" event
    let event_2 = Event::new("Confirm Payment")
        .at_ns(1_700_000_000_100_000_000)
        .by("BillingManager");
    
    println!("Events created successfully.");
```

---

## Step 4: Group Events into Traces

A `Trace` is a sequence of events sharing a common case identifier (e.g., Order ID). Traces carry events in order of occurrence.

```rust
    // Create a trace representing case "Case-1001"
    let trace = Trace::new("Case-1001", [event_1, event_2]);

    println!("Trace constructed with {} events.", trace.len());
```

---

## Step 5: Assemble the Event Log

An `EventLog` is a collection of traces. Once traces are assembled, we call `validate()` to check that the structural shape adheres to the schema bounds.

```rust
    // Assemble the event log
    let event_log = EventLog::from_traces([trace]);

    // Validate the event log structure
    match event_log.validate() {
        Ok(_) => println!("Event log structural validation passed!"),
        Err(e) => println!("Validation failed: {:?}", e),
    }
```

---

## Step 6: Stream Buffers (Optional)

If your application processes events in an online, append-only fashion before grouping them into cases, use `EventStream`:

```rust
    // Create an append-only stream buffer
    let mut stream = EventStream::new();
    
    // Stream events as they arrive
    let stream_event = Event::new("Ship Goods").at_ns(1_700_000_000_200_000_000);
    stream.push(stream_event);
    
    println!("Stream buffer holds {} event(s).", stream.len());
}
```

---

## Complete Example

Here is a full compile-passing harness:

```rust
use wasm4pm_compat::eventlog::{Event, Trace, EventLog, EventStream};

fn main() {
    let event_1 = Event::new("Register Order")
        .at_ns(1_700_000_000_000_000_000)
        .by("SystemAgent");

    let event_2 = Event::new("Confirm Payment")
        .at_ns(1_700_000_000_100_000_000)
        .by("BillingManager");

    let trace = Trace::new("Case-1001", [event_1, event_2]);

    let event_log = EventLog::from_traces([trace]);

    match event_log.validate() {
        Ok(_) => println!("Event log structural validation passed!"),
        Err(e) => println!("Validation failed: {:?}", e),
    }

    let mut stream = EventStream::new();
    let stream_event = Event::new("Ship Goods").at_ns(1_700_000_000_200_000_000);
    stream.push(stream_event);
    
    println!("Stream buffer holds {} event(s).", stream.len());
}
```

---

## Next Steps

Now that you have constructed and validated a basic case-centric event log, you can proceed to the next tutorial: [Admitting OCEL Evidence](admit-ocel-evidence.md) to learn how to validate multi-perspective event logs against standard witness laws.
