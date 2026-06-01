// COMPILE-PASS: OCPQ ObjectScopeConst const-generic scope kind — closed/open/single.
//
// Law: OcpqScopeKind encodes the binding strategy as a const generic parameter.
// ObjectScopeConst<{OcpqScopeKind::Closed}> and ObjectScopeConst<{OcpqScopeKind::Open}>
// are different types — the wrong scope kind is a compile error, not a runtime failure.
// Structure-only; scope resolution graduates to wasm4pm.
use wasm4pm_compat::ocpq::{ObjectScopeConst, OcpqScopeKind};

fn main() {
    let closed = ObjectScopeConst::<{ OcpqScopeKind::Closed }>::new(["order", "item"]);
    assert_eq!(closed.kind(), OcpqScopeKind::Closed);
    assert_eq!(closed.object_types().len(), 2);
    assert!(!closed.is_empty());

    let open = ObjectScopeConst::<{ OcpqScopeKind::Open }>::new([] as [&str; 0]);
    assert_eq!(open.kind(), OcpqScopeKind::Open);
    assert!(open.is_empty());

    let single = ObjectScopeConst::<{ OcpqScopeKind::SingleType }>::new(["order"]);
    assert_eq!(single.kind(), OcpqScopeKind::SingleType);
    assert_eq!(single.object_types(), &["order".to_string()]);
}
