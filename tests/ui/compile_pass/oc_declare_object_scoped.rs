// COMPILE-PASS: OcDeclareConstraint object-scoped construction — single type.
//
// Law: OC-Declare attaches explicit object-type annotations to a DeclareConstraint.
// A single-object-type scope is the minimal valid OC-Declare shape. The structural
// law requires object_types to be non-empty; validate() proves this.
use wasm4pm_compat::declare::{
    Activity, DeclareConstraint, DeclareScope, DeclareTemplate, OcDeclareConstraint,
};

fn main() {
    let inner = DeclareConstraint::unary(
        DeclareTemplate::Existence,
        Activity::new("pay"),
        DeclareScope::SingleObjectScope("invoice".into()),
    );
    let oc = OcDeclareConstraint::new(inner, vec!["invoice".into()]);
    assert_eq!(oc.object_types.len(), 1);
    assert!(!oc.is_synchronized());
    assert!(oc.validate().is_ok());
    assert_eq!(oc.constraint.template, DeclareTemplate::Existence);
}
