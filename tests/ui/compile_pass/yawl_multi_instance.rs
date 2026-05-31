// Compile-pass fixture: MultipleInstanceSpec can be constructed for both
// Static and Dynamic creation kinds, and validate() passes for valid bounds.
//
// Law: YAWL Definition 1 nofi: T ⇸ N × N^∞ × N^∞ × {dynamic, static}
// — every multi-instance task carries (min, max, threshold, creation_kind).

use wasm4pm_compat::petri::{MultipleInstanceSpec, InstanceCreationKind, PetriRefusal};

fn main() {
    // Static creation: 1 to 4 instances, threshold 2.
    let static_spec = MultipleInstanceSpec::new(
        1,
        Some(4),
        Some(2),
        InstanceCreationKind::Static,
    );
    assert_eq!(static_spec.min, 1);
    assert_eq!(static_spec.max, Some(4));
    assert_eq!(static_spec.creation, InstanceCreationKind::Static);
    assert!(static_spec.validate().is_ok());

    // Dynamic creation: 2 to unbounded instances, no threshold.
    let dynamic_spec = MultipleInstanceSpec::new(
        2,
        None,
        None,
        InstanceCreationKind::Dynamic,
    );
    assert_eq!(dynamic_spec.creation, InstanceCreationKind::Dynamic);
    assert_eq!(dynamic_spec.max, None); // unbounded
    assert!(dynamic_spec.validate().is_ok());

    // Exactly 1 instance (min == max) is valid.
    let single = MultipleInstanceSpec::new(1, Some(1), None, InstanceCreationKind::Static);
    assert!(single.validate().is_ok());

    // min > max is refused as InvalidInstanceBounds.
    let bad = MultipleInstanceSpec::new(5, Some(2), None, InstanceCreationKind::Static);
    assert_eq!(bad.validate(), Err(PetriRefusal::InvalidInstanceBounds));

    // min == 0 is refused as InvalidInstanceBounds (YAWL requires at least one instance).
    let zero_min = MultipleInstanceSpec::new(0, Some(3), None, InstanceCreationKind::Static);
    assert_eq!(zero_min.validate(), Err(PetriRefusal::InvalidInstanceBounds));
}
