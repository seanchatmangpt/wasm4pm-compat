// COMPILE-PASS: CompatDiagnostic::UnreachablePrimitive — proves the variant is
// constructible and used to flag canon types that are declared but wired to nothing.
//
// Law: UnreachablePrimitiveLaw — every shape the crate knows is reachable; no
// canon type is declared yet connected to no admission, projection, or export.
use wasm4pm_compat::diagnostic::CompatDiagnostic;

fn audit_reachability(is_reachable: bool) -> Option<CompatDiagnostic> {
    if !is_reachable {
        Some(CompatDiagnostic::UnreachablePrimitive)
    } else {
        None
    }
}

fn main() {
    let diag = audit_reachability(false);
    assert_eq!(diag, Some(CompatDiagnostic::UnreachablePrimitive));

    let clean = audit_reachability(true);
    assert!(clean.is_none());

    // Hash is implemented.
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(CompatDiagnostic::UnreachablePrimitive);
    assert!(set.contains(&CompatDiagnostic::UnreachablePrimitive));
}
