// COMPILE-PASS: OcDeclareConstraint object-scoped variant — structural shape
// compiles.
//
// Law: OC-Declare extends classical Declare by attaching explicit object type
// annotations to a DeclareConstraint. The object_types list must be non-empty;
// structural validation (OcDeclareRefusal::EmptyObjectTypeList) covers the
// empty case. Synchronized variants require synchronized=true.
//
// This fixture proves that:
// 1. OcDeclareConstraint::new constructs a non-synchronized constraint.
// 2. OcDeclareConstraint::synchronized constructs a synchronized constraint.
// 3. validate() accepts non-empty object_types.
// 4. validate() rejects empty object_types as EmptyObjectTypeList.
use wasm4pm_compat::declare::{
    Activity, DeclareConstraint, DeclareScope, DeclareTemplate, OcDeclareConstraint,
    OcDeclareRefusal,
};

fn main() {
    // Non-synchronized OC-Declare constraint.
    let inner = DeclareConstraint::binary(
        DeclareTemplate::Response,
        Activity::new("submit"),
        Activity::new("approve"),
        DeclareScope::SingleObjectScope("order".into()),
    );
    let oc = OcDeclareConstraint::new(inner.clone(), vec!["order".into(), "item".into()]);
    assert!(!oc.is_synchronized());
    assert!(oc.validate().is_ok());

    // Synchronized OC-Declare constraint.
    let sync_inner = DeclareConstraint::binary(
        DeclareTemplate::ChainSuccession,
        Activity::new("ship"),
        Activity::new("deliver"),
        DeclareScope::SynchronizedObjectScope(vec!["order".into(), "delivery".into()]),
    );
    let sync_oc = OcDeclareConstraint::synchronized(
        sync_inner,
        vec!["order".into(), "delivery".into()],
    );
    assert!(sync_oc.is_synchronized());
    assert!(sync_oc.validate().is_ok());

    // Empty object type list is refused.
    let bad = OcDeclareConstraint::new(inner, vec![]);
    assert_eq!(bad.validate(), Err(OcDeclareRefusal::EmptyObjectTypeList));

    // Extended templates are constructible.
    let oc_chain = DeclareConstraint::binary(
        DeclareTemplate::AlternateSuccession,
        Activity::new("create"),
        Activity::new("close"),
        DeclareScope::MultiObjectScope(vec!["order".into(), "item".into()]),
    );
    let oc_alt = OcDeclareConstraint::new(oc_chain, vec!["order".into()]);
    assert_eq!(oc_alt.constraint.template, DeclareTemplate::AlternateSuccession);
    assert_eq!(oc_alt.constraint.template.arity(), 2);

    // Init template is unary and constructible.
    let init_c = DeclareConstraint::unary(
        DeclareTemplate::Init,
        Activity::new("start"),
        DeclareScope::SingleObjectScope("case".into()),
    );
    let oc_init = OcDeclareConstraint::new(init_c, vec!["case".into()]);
    assert_eq!(oc_init.constraint.template.arity(), 1);
}
