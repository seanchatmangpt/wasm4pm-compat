// COMPILE-PASS: DeclareConstraint::Response binary template — lawful construction.
//
// Law: Response(a, b) asserts that every occurrence of a is eventually followed
// by b. This is a binary template (arity 2) and requires both an activation and
// a target activity. Structural shape only — no evaluation against a log.
use wasm4pm_compat::declare::{Activity, DeclareConstraint, DeclareScope, DeclareTemplate};

fn main() {
    let c = DeclareConstraint::binary(
        DeclareTemplate::Response,
        Activity::new("submit"),
        Activity::new("approve"),
        DeclareScope::SingleObjectScope("order".into()),
    );
    assert_eq!(c.template, DeclareTemplate::Response);
    assert_eq!(c.template.arity(), 2);
    assert!(!c.template.is_negative());
    assert!(!c.template.is_chain());
    assert!(c.target.is_some());
    assert_eq!(c.activation.0, "submit");
}
