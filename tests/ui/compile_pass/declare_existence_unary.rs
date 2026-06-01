// COMPILE-PASS: DeclareConstraint::Existence unary template — lawful construction.
//
// Law: Existence(a) asserts that a occurs at least once. Unary template
// (arity 1), positive constraint. Existence2 and Existence3 are also unary.
// Structural shape only — no evaluation.
use wasm4pm_compat::declare::{Activity, DeclareConstraint, DeclareScope, DeclareTemplate};

fn main() {
    let c = DeclareConstraint::unary(
        DeclareTemplate::Existence,
        Activity::new("pay"),
        DeclareScope::SingleObjectScope("invoice".into()),
    );
    assert_eq!(c.template, DeclareTemplate::Existence);
    assert_eq!(c.template.arity(), 1);
    assert!(!c.template.is_negative());
    assert!(c.target.is_none());

    let c2 = DeclareConstraint::unary(
        DeclareTemplate::Existence2,
        Activity::new("review"),
        DeclareScope::SingleObjectScope("document".into()),
    );
    assert_eq!(c2.template.arity(), 1);
    assert!(!c2.template.is_negative());

    let c3 = DeclareConstraint::unary(
        DeclareTemplate::Existence3,
        Activity::new("retry"),
        DeclareScope::SingleObjectScope("job".into()),
    );
    assert_eq!(c3.template.arity(), 1);
    assert!(!c3.template.is_negative());
}
