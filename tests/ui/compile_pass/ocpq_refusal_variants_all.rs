// COMPILE-PASS: OCPQ OcpqRefusal named law variants — all variants construct.
//
// Law: OCPQ refusal law names specific structural violations — never bare
// "InvalidInput". All ten named variants must be constructible and display the
// named law.
use wasm4pm_compat::ocpq::OcpqRefusal;

fn main() {
    let variants = [
        OcpqRefusal::MissingObjectScope,
        OcpqRefusal::UnknownObjectType,
        OcpqRefusal::UnknownEventType,
        OcpqRefusal::InvalidCardinality,
        OcpqRefusal::UnsafeProjection,
        OcpqRefusal::FlatteningRequired,
        OcpqRefusal::InvalidChildSetBound,
        OcpqRefusal::EmptyScopeType,
        OcpqRefusal::ConflictingPredicateKinds,
        OcpqRefusal::UnboundVariable,
    ];

    for v in &variants {
        // Each variant's Display starts with "OCPQ refused: ".
        assert!(v.to_string().starts_with("OCPQ refused: "));
    }

    // Spot-check specific display values.
    assert_eq!(OcpqRefusal::FlatteningRequired.to_string(), "OCPQ refused: FlatteningRequired");
    assert_eq!(OcpqRefusal::InvalidChildSetBound.to_string(), "OCPQ refused: InvalidChildSetBound");
    assert_eq!(OcpqRefusal::UnboundVariable.to_string(), "OCPQ refused: UnboundVariable");
}
