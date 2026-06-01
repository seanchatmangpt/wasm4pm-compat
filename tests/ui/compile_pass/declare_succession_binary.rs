// COMPILE-PASS: DeclareConstraint::Succession binary template — lawful construction.
//
// Law: Succession(a, b) is both Response and Precedence: a must eventually be
// followed by b AND b must be preceded by a. Binary template (arity 2).
// Structural shape only — no evaluation.
use wasm4pm_compat::declare::{Activity, DeclareConstraint, DeclareScope, DeclareTemplate};

fn main() {
    let c = DeclareConstraint::binary(
        DeclareTemplate::Succession,
        Activity::new("create"),
        Activity::new("close"),
        DeclareScope::MultiObjectScope(vec!["order".into(), "item".into()]),
    );
    assert_eq!(c.template, DeclareTemplate::Succession);
    assert_eq!(c.template.arity(), 2);
    assert!(!c.template.is_negative());
    assert!(!c.template.is_chain());
    assert!(c.target.is_some());
    assert!(matches!(c.scope, DeclareScope::MultiObjectScope(_)));
}
