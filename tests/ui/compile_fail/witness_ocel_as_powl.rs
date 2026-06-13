// COMPILE-FAIL: Witness confusion law — Evidence<T, Admitted, Ocel20> cannot be used
// where Evidence<T, Admitted, PowlPaper> is required.
// Law: The witness type parameter W makes OCEL-admitted evidence and POWL-admitted
// evidence distinct types with no coercion path between them.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Admitted;
use wasm4pm_compat::witness::{Ocel20, PowlPaper};

fn requires_powl_evidence(_: Evidence<String, Admitted, PowlPaper>) {}

fn _test(ocel_ev: Evidence<String, Admitted, Ocel20>) {
    // This must fail: Ocel20 witness is not PowlPaper witness.
    requires_powl_evidence(ocel_ev);
}

fn main() {}
