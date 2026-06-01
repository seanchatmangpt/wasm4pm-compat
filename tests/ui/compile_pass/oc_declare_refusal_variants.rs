// COMPILE-PASS: OcDeclareRefusal named law variants — all three variants construct.
//
// Law: OC-Declare refusal law names specific structural violations rather than
// bare "InvalidInput". All three variants must be constructible: EmptyObjectTypeList,
// SynchronizationRequiresMultipleTypes, ScopeMismatch.
use wasm4pm_compat::declare::OcDeclareRefusal;

fn main() {
    let r1 = OcDeclareRefusal::EmptyObjectTypeList;
    let r2 = OcDeclareRefusal::SynchronizationRequiresMultipleTypes;
    let r3 = OcDeclareRefusal::ScopeMismatch;

    assert_eq!(r1.to_string(), "OcDeclare refused: EmptyObjectTypeList");
    assert_eq!(r2.to_string(), "OcDeclare refused: SynchronizationRequiresMultipleTypes");
    assert_eq!(r3.to_string(), "OcDeclare refused: ScopeMismatch");

    // Refusals are structural values — Clone and Debug are implemented.
    let _r1c = r1.clone();
    let _r2c = r2.clone();
    let _r3c = r3.clone();
}
