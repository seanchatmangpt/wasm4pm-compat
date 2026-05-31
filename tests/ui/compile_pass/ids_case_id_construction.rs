// COMPILE-PASS: CaseId — XES parse-boundary case identifier.
//
// Law: CaseId<K> names a case attribute as parsed from an external format
// (e.g. XES concept:name). It is structurally distinct from TraceId<K>, which
// names a structural trace position within an admitted EventLog. Mixing them is
// a compile error, not a naming convention.
use wasm4pm_compat::ids::{CaseId, TypedId};

enum MyLog {}

fn main() {
    let case = CaseId::<MyLog>::new(7u64);
    assert_eq!(case.raw(), 7u64);
    assert!(!case.is_zero());

    let zero = CaseId::<MyLog>::new(0u64);
    assert!(zero.is_zero());

    // Copy semantics work without K: Clone.
    let case2 = case;
    assert_eq!(case, case2);

    // Display includes the type name — no kind erasure.
    assert_eq!(format!("{}", case), "CaseId(7)");

    // raw_value() via TypedId trait.
    assert_eq!(case.raw_value(), 7u64);
}
