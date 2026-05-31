// COMPILE-FAIL: One-way-door law — raw evidence cannot be used as admitted.
// Law: Evidence<T, Raw, W>::into_inner() does not exist; only Admitted has it.
// Expected error: no method `into_inner` found for Evidence<u32, Raw, Ocel20>.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::witness::Ocel20;

fn main() {
    let raw = Evidence::<u32, _, Ocel20>::raw(42u32);
    // into_inner() is only on Evidence<T, Admitted, W>; Raw does not have it.
    let _ = raw.into_inner();
}
