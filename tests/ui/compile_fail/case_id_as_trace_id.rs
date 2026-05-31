// COMPILE-FAIL: Kind-typed ID law — CaseId cannot be passed where TraceId is
// required, even though both wrap a raw u64.
//
// Law: CaseId<K> names a case attribute at the XES parse boundary; TraceId<K>
// names a structural trace position inside an admitted EventLog. They are
// distinct newtypes — the compiler must reject substitution, not allow it
// silently at runtime.
use wasm4pm_compat::ids::{CaseId, TraceId};

enum MyLog {}

fn requires_trace_id(_id: TraceId<MyLog>) {}

fn main() {
    let case: CaseId<MyLog> = CaseId::new(99u64);
    // This must fail: CaseId<MyLog> is not TraceId<MyLog>.
    requires_trace_id(case);
}
