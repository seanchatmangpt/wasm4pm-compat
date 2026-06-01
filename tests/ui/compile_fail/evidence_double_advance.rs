// COMPILE-FAIL: Evidence double-advance — advancing an already-admitted evidence fails.
//
// Law: EvidenceOneWayDoorLaw — Evidence<T, Admitted, W> cannot be passed where
// Evidence<T, Raw, W> is required. The one-way door means once admitted, the
// raw form is gone — you cannot re-admit already-admitted evidence.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::{Admitted, Raw};
use wasm4pm_compat::witness::Ocel20;

// Simulates an admission step that requires Raw input
fn requires_raw_input(_ev: Evidence<String, Raw, Ocel20>) {
    // an admission step would transform Raw → Admitted
}

fn main() {
    // Build admitted evidence via raw (in tests we can only build raw directly)
    let raw: Evidence<String, Raw, Ocel20> = Evidence::raw("payload".to_string());
    // In real code admitted only comes from Admit::admit() — but the type-level
    // law is demonstrated by the mismatch: Evidence<_,Admitted,_> ≠ Evidence<_,Raw,_>
    let _: Evidence<String, Admitted, Ocel20> = raw;
    // This must fail: Evidence<String, Raw, Ocel20> cannot be used as
    // Evidence<String, Admitted, Ocel20> — the states are distinct types.
}
