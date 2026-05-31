// COMPILE-PASS: CaseId and TraceId are distinct types — parse-boundary vs. admitted-log law.
//
// Law: CaseId<K> names a case attribute at the XES parse boundary.
// TraceId<K> names a structural trace position within an admitted EventLog.
// Both wrap u64 but are intentionally distinct types. Mixing them is a compile
// error, not a naming convention — this prevents the parse-boundary / admitted
// confusion that causes dangling link bugs.
use wasm4pm_compat::ids::{CaseId, TraceId, TypedId};

enum MyLog {}

fn require_trace_id(_: TraceId<MyLog>) {}
fn require_case_id(_: CaseId<MyLog>) {}

fn main() {
    let case = CaseId::<MyLog>::new(5u64);
    let trace = TraceId::<MyLog>::new(5u64);

    // Same raw value, different types.
    assert_eq!(case.raw(), 5u64);
    assert_eq!(trace.raw(), 5u64);

    // Kind-safe acceptance.
    require_trace_id(trace);
    require_case_id(case);
    // `require_trace_id(case)` — compile error: CaseId is not TraceId.

    // Both are is_zero()-aware via TypedId.
    assert!(!case.is_zero());
    assert!(!trace.is_zero());

    // Zero sentinels are distinct types too.
    let zero_case = CaseId::<MyLog>::new(0u64);
    let zero_trace = TraceId::<MyLog>::new(0u64);
    assert!(zero_case.is_zero());
    assert!(zero_trace.is_zero());
}
