// COMPILE-FAIL: Kind-typed ID law — CaseId cannot be passed where ActivityId is required.
// Law: CaseId<K> wraps u64; ActivityId<K> wraps u32. They are distinct newtypes.
// Passing a case id as an activity id is a compile error.
use wasm4pm_compat::ids::{ActivityId, CaseId};

enum MyLog {}

fn requires_activity_id(_id: ActivityId<MyLog>) {}

fn main() {
    let case: CaseId<MyLog> = CaseId::new(1u64);
    // This must fail: CaseId<MyLog> is not ActivityId<MyLog>.
    requires_activity_id(case);
}
