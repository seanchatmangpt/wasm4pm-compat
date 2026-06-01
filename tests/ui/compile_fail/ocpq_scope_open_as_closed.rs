#![feature(adt_const_params)]
#![allow(incomplete_features)]
// COMPILE-FAIL: OCPQ scope law — ObjectScopeConst<{Open}> cannot be passed where
// ObjectScopeConst<{Closed}> is required.
// Law: OcpqScopeKind is a const generic; Open and Closed produce distinct types.
// A function requiring a closed scope rejects an open scope at compile time.
use wasm4pm_compat::ocpq::{ObjectScopeConst, OcpqScopeKind};

fn requires_closed_scope(_s: ObjectScopeConst<{ OcpqScopeKind::Closed }>) {}

fn main() {
    let open = ObjectScopeConst::<{ OcpqScopeKind::Open }>::new(["order"]);
    // This must fail: Open scope is not Closed scope.
    requires_closed_scope(open);
}
