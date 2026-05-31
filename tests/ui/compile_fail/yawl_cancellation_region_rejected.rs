// COMPILE-FAIL: CancellationRegionExclusionLaw — raw Vec<String> cannot be
// passed where a CancellationRegion is required.
//
// Law: YAWL Definition 1 rem: T ⇸ P(T ∪ C \ {i,o}) — the cancellation region
// is a named, typed shape. The #[repr(transparent)] CancellationRegion newtype
// prevents a bare Vec<String> from being substituted for a CancellationRegion.
// A function that requires a CancellationRegion must reject a raw vector even
// when the vector would happen to contain only lawful (non-{i,o}) ids.
//
// Expected error: mismatched types — found Vec<String>, expected CancellationRegion.
use wasm4pm_compat::petri::CancellationRegion;

fn attach_region(_region: CancellationRegion) {}

fn main() {
    // A bare Vec<String> — even one that excludes the initial/final places —
    // must not compile as a CancellationRegion. The type system enforces that
    // only the CancellationRegion constructor may name a cancellation set.
    let raw: Vec<String> = vec!["p2".to_string(), "t1".to_string()];
    // ERROR: Vec<String> is not CancellationRegion.
    attach_region(raw);
}
