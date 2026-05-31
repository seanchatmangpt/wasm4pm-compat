// COMPILE-FAIL: Rejects assigning Evidence<T, Raw, W> where Evidence<T, Admitted, W> is required —
// proves the one-way door is enforced at the type level with no coercion.
//
// Law: Evidence<T, Raw, W> and Evidence<T, Admitted, W> are distinct types. A function
// that demands admitted evidence cannot be called with raw evidence — no conversion
// or coercion is available. The only path to Admitted is through an Admit impl.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::{Admitted, Raw};
use wasm4pm_compat::witness::Ocel20;

fn requires_admitted(_ev: Evidence<String, Admitted, Ocel20>) {}

fn main() {
    let raw: Evidence<String, Raw, Ocel20> = Evidence::raw("untrusted".to_string());
    // This must fail: Evidence<String, Raw, Ocel20> is not Evidence<String, Admitted, Ocel20>.
    // The Admitted constructor is pub(crate); there is no public coercion path.
    requires_admitted(raw);
}
