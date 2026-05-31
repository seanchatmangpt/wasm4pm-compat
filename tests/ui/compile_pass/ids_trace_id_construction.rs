// COMPILE-PASS: TraceId — structural trace-position identifier in an admitted EventLog.
//
// Law: TraceId<K> is a #[repr(transparent)] newtype over u64. It names a
// structural trace position within an already-admitted EventLog. It is
// intentionally distinct from CaseId<K> (which names a case attribute at the
// XES parse boundary). Both wrap u64, but they are different types — mixing
// them is a compile error.
use wasm4pm_compat::ids::{TraceId, TypedId};

enum MyLog {}

fn main() {
    let trace = TraceId::<MyLog>::new(3u64);
    assert_eq!(trace.raw(), 3u64);
    assert!(!trace.is_zero());

    let zero = TraceId::<MyLog>::new(0u64);
    assert!(zero.is_zero());

    // Ord: by raw value.
    let trace2 = TraceId::<MyLog>::new(4u64);
    assert!(trace < trace2);

    // Display shows the type name.
    assert_eq!(format!("{}", trace), "TraceId(3)");

    // raw_value() round-trip via trait.
    assert_eq!(trace.raw_value(), 3u64);

    // Copy semantics.
    let copy = trace;
    assert_eq!(copy, trace);
}
