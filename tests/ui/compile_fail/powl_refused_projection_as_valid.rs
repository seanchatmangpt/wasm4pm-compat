// COMPILE-FAIL: RefusedProjectionForwardedAsValid — RefusedProjection cannot satisfy TreeProjectable
// Law: POWL projection law — a refused projection carries a verdict, not a valid projection marker.
// A RefusedProjection must not be forwarded to a gate that requires ProcessTreeProjectable.
// Expected error: type mismatch — RefusedProjection is not TreeProjectable.
use wasm4pm_compat::powl::{assert_tree_projectable, RefusedProjection, PowlRefusal};

fn main() {
    let refused = RefusedProjection::new(PowlRefusal::IrreducibleProjection);
    assert_tree_projectable(refused);
}
