// Law: MarkingConstructionLaw — Marking is constructible as empty or with token assignments; tokens_on() returns 0 for absent places; structure-only with no token dynamics
// COMPILE-PASS: Marking — empty and non-empty markings are constructible.
// tokens_on() returns 0 for absent places. Structure-only: no token dynamics.
use wasm4pm_compat::petri::Marking;

fn main() {
    let empty = Marking::empty();
    assert!(empty.is_empty());
    assert_eq!(empty.tokens_on("p0"), 0);

    let m = Marking::new([("p0".to_string(), 1), ("p1".to_string(), 3)]);
    assert_eq!(m.tokens_on("p0"), 1);
    assert_eq!(m.tokens_on("p1"), 3);
    assert_eq!(m.tokens_on("p_unknown"), 0);
    assert!(!m.is_empty());
}
