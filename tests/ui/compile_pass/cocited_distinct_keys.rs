// COMPILE-PASS: Co-citation law — distinct witness keys co-cite lawfully.
//
// Law: CoCitedKey<T, K1, K2> compiles iff K1 != K2 (Assert<{ !str_eq(K1,K2) }>:
// IsTrue). Proves the lawful path is open: two distinct authority keys may be
// co-cited over a value.
#![feature(generic_const_exprs, adt_const_params, unsized_const_params)]
#![allow(incomplete_features)]
use wasm4pm_compat::witness_law::CoCitedKey;

fn main() {
    // Distinct keys: lawful co-citation.
    let cited = CoCitedKey::<u32, "ocel-2.0", "alpha-miner">::new(42);
    assert_eq!(cited.value, 42);
}
