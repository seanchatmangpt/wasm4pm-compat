// Law: ArcWeightBuilderLaw — Arc::with_weight() sets the arc multiplicity via builder pattern; the weight is structurally recorded, not dynamically fired
// COMPILE-PASS: Arc::with_weight — builder-style weight setting compiles and
// produces the expected arc multiplicity. Structure-only.
use wasm4pm_compat::petri::Arc;

fn main() {
    let arc = Arc::place_to_transition("p0", "t0").with_weight(5);
    assert_eq!(arc.weight(), 5);

    let arc2 = Arc::transition_to_place("t0", "p1").with_weight(2);
    assert_eq!(arc2.weight(), 2);
}
