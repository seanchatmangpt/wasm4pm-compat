// COMPILE-PASS: DeclareConstraint::Precedence binary template — lawful construction.
//
// Law: Precedence(a, b) asserts that every occurrence of b is preceded by a.
// Binary template (arity 2). Structural shape only — no evaluation.
use wasm4pm_compat::declare::{Activity, DeclareConstraint, DeclareScope, DeclareTemplate};

fn main() {
    let c = DeclareConstraint::binary(
        DeclareTemplate::Precedence,
        Activity::new("pay"),
        Activity::new("ship"),
        DeclareScope::SingleObjectScope("order".into()),
    );
    assert_eq!(c.template, DeclareTemplate::Precedence);
    assert_eq!(c.template.arity(), 2);
    assert!(!c.template.is_negative());
    assert!(c.target.is_some());
}
