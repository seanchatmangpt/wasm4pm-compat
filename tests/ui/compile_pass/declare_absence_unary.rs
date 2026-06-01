// COMPILE-PASS: DeclareConstraint::Absence unary template — lawful construction.
//
// Law: Absence(a) asserts that a does not occur. Unary template (arity 1) and
// a negative constraint (is_negative returns true). No target activity.
// Structural shape only — no evaluation.
use wasm4pm_compat::declare::{Activity, DeclareConstraint, DeclareScope, DeclareTemplate};

fn main() {
    let c = DeclareConstraint::unary(
        DeclareTemplate::Absence,
        Activity::new("cancel"),
        DeclareScope::SingleObjectScope("order".into()),
    );
    assert_eq!(c.template, DeclareTemplate::Absence);
    assert_eq!(c.template.arity(), 1);
    assert!(c.template.is_negative());
    assert!(!c.template.is_chain());
    assert!(c.target.is_none());

    // Absence2 and Absence3 are also negative unary templates.
    let c2 = DeclareConstraint::unary(
        DeclareTemplate::Absence2,
        Activity::new("reject"),
        DeclareScope::SingleObjectScope("claim".into()),
    );
    assert_eq!(c2.template.arity(), 1);
    assert!(c2.template.is_negative());

    let c3 = DeclareConstraint::unary(
        DeclareTemplate::Absence3,
        Activity::new("escalate"),
        DeclareScope::SingleObjectScope("ticket".into()),
    );
    assert_eq!(c3.template.arity(), 1);
    assert!(c3.template.is_negative());
}
