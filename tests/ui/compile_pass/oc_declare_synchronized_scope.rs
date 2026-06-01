// COMPILE-PASS: OcDeclareConstraint synchronized scope — joint lifecycle construction.
//
// Law: OC-Declare synchronized constraints require multiple object types to share
// a joint lifecycle. The synchronized=true flag marks this structurally. Evaluation
// of joint lifecycle synchronization graduates to wasm4pm.
use wasm4pm_compat::declare::{
    Activity, DeclareConstraint, DeclareScope, DeclareTemplate, OcDeclareConstraint,
};

fn main() {
    let inner = DeclareConstraint::binary(
        DeclareTemplate::Succession,
        Activity::new("ship"),
        Activity::new("deliver"),
        DeclareScope::SynchronizedObjectScope(vec!["order".into(), "delivery".into()]),
    );
    let oc = OcDeclareConstraint::synchronized(
        inner,
        vec!["order".into(), "delivery".into()],
    );
    assert!(oc.is_synchronized());
    assert_eq!(oc.object_types.len(), 2);
    assert!(oc.validate().is_ok());
    assert_eq!(oc.constraint.template, DeclareTemplate::Succession);
}
