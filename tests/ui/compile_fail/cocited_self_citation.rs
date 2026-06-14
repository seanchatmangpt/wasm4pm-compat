// COMPILE-FAIL: Co-citation law — a witness cannot co-cite itself.
//
// Law: CoCitationDistinctnessLaw — CoCitedKey<T, K1, K2> requires
// Assert<{ !str_eq(K1, K2) }>: IsTrue. When K1 == K2 the const string-equality
// law evaluates to false, so the bound is unsatisfiable and construction is a
// compile error. This is the receipt that the co-citation law is ENFORCED, not
// merely named: self-citation is unrepresentable at the type level.
//
// Expected error: E0308 — expected constant `false`, found constant `true`
// (the !str_eq(K1, K2) bound resolves false for identical keys).
#![feature(generic_const_exprs, adt_const_params, unsized_const_params)]
#![allow(incomplete_features)]
use wasm4pm_compat::witness_law::CoCitedKey;

fn main() {
    // Same key on both sides: self-citation. Must NOT compile.
    let _forged = CoCitedKey::<u32, "ocel-2.0", "ocel-2.0">::new(1);
}
