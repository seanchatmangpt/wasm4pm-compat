// COMPILE-PASS: OCPQ OcpqQueryConst with event predicate added via builder — lawful.
//
// Law: OcpqQueryConst::with_predicate adds predicates to the query body builder-style.
// The predicate witness type is erased at the collection level (Predicate<()>) so the
// query body accepts all predicate families. Structure-only; evaluation graduates to wasm4pm.
use wasm4pm_compat::ocpq::{
    ObjectScopeConst, OcpqQueryConst, OcpqScopeKind, Predicate, PredicateKind,
};

fn main() {
    // with_predicate accepts Predicate<()> (the default); witness is erased at the collection level.
    let q = OcpqQueryConst::<{ OcpqScopeKind::Closed }>::new(
        ObjectScopeConst::<{ OcpqScopeKind::Closed }>::new(["order"]),
    )
    .with_predicate(Predicate::new(
        PredicateKind::Event("activity = pay".into()),
    ));
    assert_eq!(q.predicates().len(), 1);
    assert!(matches!(q.predicates()[0].kind, PredicateKind::Event(_)));
}
