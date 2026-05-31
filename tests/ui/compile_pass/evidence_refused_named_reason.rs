// COMPILE-PASS: Evidence<&str, Refused, Ocel20> with named reason
//
// Law: Refused is first-class and terminal. The refusal reason is a named
// type — never a bare catch-all. This fixture proves:
//   1. The Parsed → Refused path compiles via into_refused().
//   2. as_refused_value() gives back the original value for diagnostics.
//   3. Refusal<R, W>::new(reason) carries the named reason through the type.
use wasm4pm_compat::admission::Refusal;
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Refused;
use wasm4pm_compat::witness::Ocel20;

/// Named refusal law: the OCEL log carries no events.
#[derive(Debug, PartialEq)]
enum NoEventsInLog {
    Violation,
}

fn main() {
    // Parsed → Refused path: well-formed bytes, refused for structural law.
    let parsed = Evidence::<_, _, Ocel20>::raw("ocel-no-events").into_parsed();
    let refused: Evidence<&str, Refused, Ocel20> = parsed.into_refused();
    assert_eq!(*refused.as_refused_value(), "ocel-no-events");

    // Refusal<R, W> carries the named reason, not a bare string.
    let verdict = Refusal::<_, Ocel20>::new(NoEventsInLog::Violation);
    assert_eq!(verdict.reason, NoEventsInLog::Violation);
}
