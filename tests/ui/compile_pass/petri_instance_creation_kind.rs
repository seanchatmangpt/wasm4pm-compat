// Law: InstanceCreationKindLaw — InstanceCreationKind has exactly two YAWL nofi variants (Static, Dynamic); both are Copy and mutually distinguishable
// COMPILE-PASS: InstanceCreationKind — the Static and Dynamic variants of the
// YAWL nofi creation kind are constructible, Copy, and distinguishable.
use wasm4pm_compat::petri::InstanceCreationKind;

fn main() {
    let s = InstanceCreationKind::Static;
    let d = InstanceCreationKind::Dynamic;
    assert_ne!(s, d);
    assert_eq!(s, InstanceCreationKind::Static);
    assert_eq!(d, InstanceCreationKind::Dynamic);
    // Copy semantics
    let s2 = s;
    assert_eq!(s2, InstanceCreationKind::Static);
}
