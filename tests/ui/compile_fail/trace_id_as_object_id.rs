// COMPILE-FAIL: Kind-typed ID law — TraceId cannot be passed where ObjectId is required.
// Law: TraceId<K> and ObjectId<K> both wrap u64 but are distinct newtypes.
// A trace id (XES/case-centric) must never be confused with an object id (OCEL).
use wasm4pm_compat::ids::{ObjectId, TraceId};

enum MyLog {}

fn requires_object_id(_id: ObjectId<MyLog>) {}

fn main() {
    let trace: TraceId<MyLog> = TraceId::new(5u64);
    // This must fail: TraceId<MyLog> is not ObjectId<MyLog>.
    requires_object_id(trace);
}
