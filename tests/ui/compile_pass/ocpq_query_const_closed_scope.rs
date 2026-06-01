// COMPILE-PASS: OCPQ OcpqQueryConst closed-scope query — lawful construction.
//
// Law: OcpqQueryConst<{OcpqScopeKind::Closed}> requires a closed-scope
// ObjectScopeConst at the type level. A function requiring a closed-scope query
// rejects an open-scope query at compile time, not at runtime.
// Structure-only; query evaluation graduates to wasm4pm.
use wasm4pm_compat::ocpq::{ObjectScopeConst, OcpqQueryConst, OcpqScopeKind};

fn main() {
    let q = OcpqQueryConst::<{ OcpqScopeKind::Closed }>::new(
        ObjectScopeConst::<{ OcpqScopeKind::Closed }>::new(["order", "item"]),
    );
    assert_eq!(q.scope_kind(), OcpqScopeKind::Closed);
    assert_eq!(q.scope().object_types(), &["order".to_string(), "item".to_string()]);
    assert!(q.predicates().is_empty());
    assert!(q.sub_queries().is_empty());
}
