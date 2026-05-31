// COMPILE-PASS: Cross-namespace kind-marker law — same id type, different K is a different type.
//
// Law: The K kind marker on each typed id stamps the id with a namespace
// (e.g. a log identity or witness). EventId<LogA> and EventId<LogB> are
// different types even though both wrap u64 with the same raw value.
// This prevents ids from different logs from being confused at the type level.
use wasm4pm_compat::ids::{EventId, ObjectId};

enum LogA {}
enum LogB {}

fn require_log_a_event(_: EventId<LogA>) {}
fn require_log_b_event(_: EventId<LogB>) {}

fn main() {
    let ev_a = EventId::<LogA>::new(7u64);
    let ev_b = EventId::<LogB>::new(7u64);

    // Same raw value — different types.
    assert_eq!(ev_a.raw(), 7u64);
    assert_eq!(ev_b.raw(), 7u64);

    // Kind-safe: each function only accepts its namespace.
    require_log_a_event(ev_a);
    require_log_b_event(ev_b);
    // `require_log_a_event(ev_b)` — compile error: EventId<LogB> is not EventId<LogA>.

    // Namespace cross-check also applies to ObjectId.
    let obj_a = ObjectId::<LogA>::new(3u64);
    let obj_b = ObjectId::<LogB>::new(3u64);
    fn require_obj_b(_: ObjectId<LogB>) {}
    require_obj_b(obj_b);
    // `require_obj_b(obj_a)` — compile error.
    let _ = obj_a;
}
