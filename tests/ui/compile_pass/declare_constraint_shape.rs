// COMPILE-PASS: DeclareConstraint shape — unary and binary constraints compile.
//
// Law: Declare template arity is a structural law: binary templates require
// both activation and target activities; unary templates require only activation.
// The type system does not enforce this at the constructor level, but the
// DeclareRefusal::InvalidTemplateArity law covers runtime admission refusal.
use wasm4pm_compat::declare::{Activity, DeclareConstraint, DeclareScope, DeclareTemplate};

fn main() {
    // Unary constraint: Existence(approve)
    let existence = DeclareConstraint::unary(
        DeclareTemplate::Existence,
        Activity::new("approve"),
        DeclareScope::SingleObjectScope("order".into()),
    );
    assert_eq!(existence.template.arity(), 1);
    assert!(existence.target.is_none());

    // Binary constraint: Response(approve, notify)
    let response = DeclareConstraint::binary(
        DeclareTemplate::Response,
        Activity::new("approve"),
        Activity::new("notify"),
        DeclareScope::SingleObjectScope("case".into()),
    );
    assert_eq!(response.template.arity(), 2);
    assert!(response.target.is_some());

    // OC-Declare: Precedence scoped to multiple object types
    let oc_precedence = DeclareConstraint::binary(
        DeclareTemplate::Precedence,
        Activity::new("submit"),
        Activity::new("approve"),
        DeclareScope::MultiObjectScope(vec!["order".into(), "item".into()]),
    );
    assert!(matches!(oc_precedence.scope, DeclareScope::MultiObjectScope(_)));

    // Synchronized scope: joint lifecycle constraint
    let sync = DeclareConstraint::binary(
        DeclareTemplate::Succession,
        Activity::new("ship"),
        Activity::new("deliver"),
        DeclareScope::SynchronizedObjectScope(vec!["order".into(), "delivery".into()]),
    );
    assert!(matches!(sync.scope, DeclareScope::SynchronizedObjectScope(_)));
}
