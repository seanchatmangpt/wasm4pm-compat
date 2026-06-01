// Law: MultipleInstanceSpecValidationLaw — MultipleInstanceSpec validates 1 <= min <= max at runtime; structural law for YAWL nofi four-tuple is enforced
// COMPILE-PASS: MultipleInstanceSpec — the YAWL nofi four-tuple is constructible
// and validates the 1 ≤ min ≤ max structural law. Structure-only.
use wasm4pm_compat::petri::{MultipleInstanceSpec, InstanceCreationKind};

fn main() {
    let spec = MultipleInstanceSpec::new(1, Some(4), Some(2), InstanceCreationKind::Static);
    assert!(spec.validate().is_ok());
    assert_eq!(spec.min, 1);
    assert_eq!(spec.max, Some(4));
    assert_eq!(spec.creation, InstanceCreationKind::Static);

    let dynamic = MultipleInstanceSpec::new(2, None, None, InstanceCreationKind::Dynamic);
    assert!(dynamic.validate().is_ok());
    assert_eq!(dynamic.creation, InstanceCreationKind::Dynamic);
}
