// COMPILE-FAIL: Kind-typed ID law — ActivityId cannot be passed where CaseId is required.
// Law: ActivityId<K> wraps u32; CaseId<K> wraps u64. They are distinct newtypes.
// Passing an activity id as a case id is a compile error.
use wasm4pm_compat::ids::{ActivityId, CaseId};

enum MyLog {}

fn requires_case_id(_id: CaseId<MyLog>) {}

fn main() {
    let act: ActivityId<MyLog> = ActivityId::new(7u32);
    // This must fail: ActivityId<MyLog> is not CaseId<MyLog>.
    requires_case_id(act);
}
