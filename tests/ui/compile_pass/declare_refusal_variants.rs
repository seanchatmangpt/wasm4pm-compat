// COMPILE-PASS: DeclareRefusal named law variants — all five variants construct.
//
// Law: Declare refusal law names specific structural violations. All five named
// laws must be constructible: MissingActivation, MissingTarget,
// InvalidTemplateArity, EmptyObjectScope, SynchronizationViolation.
use wasm4pm_compat::declare::DeclareRefusal;

fn main() {
    let r1 = DeclareRefusal::MissingActivation;
    let r2 = DeclareRefusal::MissingTarget;
    let r3 = DeclareRefusal::InvalidTemplateArity;
    let r4 = DeclareRefusal::EmptyObjectScope;
    let r5 = DeclareRefusal::SynchronizationViolation;

    assert_eq!(r1.to_string(), "Declare refused: MissingActivation");
    assert_eq!(r2.to_string(), "Declare refused: MissingTarget");
    assert_eq!(r3.to_string(), "Declare refused: InvalidTemplateArity");
    assert_eq!(r4.to_string(), "Declare refused: EmptyObjectScope");
    assert_eq!(r5.to_string(), "Declare refused: SynchronizationViolation");

    let _r1c = r1.clone();
    let _r2c = r2.clone();
    let _r3c = r3.clone();
    let _r4c = r4.clone();
    let _r5c = r5.clone();
}
