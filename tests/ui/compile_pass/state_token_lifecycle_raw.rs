// COMPILE-PASS: STATE_TOKEN_RAW — Raw is a distinct zero-sized lifecycle marker
//
// Law: Raw is the entry stage for untrusted input. Evidence<T, Raw, W> can be
// freely constructed via Evidence::raw. Raw is an uninhabited empty enum used
// only as a PhantomData tag — it has zero runtime cost and is distinct from
// every other state token.
use core::marker::PhantomData;

use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Raw;
use wasm4pm_compat::witness::Ocel20;

fn main() {
    // Raw evidence can be freely constructed from any value.
    let raw: Evidence<u32, Raw, Ocel20> = Evidence::raw(42u32);
    assert_eq!(raw.value, 42u32);

    // The state field is zero-sized PhantomData — no heap allocation.
    let _: PhantomData<Raw> = raw.state;

    // Raw is Copy + Clone (the token itself is uninhabited; PhantomData is always Copy).
    let raw2: Evidence<&str, Raw, Ocel20> = Evidence::raw("untrusted bytes");
    assert_eq!(raw2.value, "untrusted bytes");

    // Raw → Parsed is the only forward transition available on Evidence<T, Raw, W>.
    let parsed = raw2.into_parsed();
    assert_eq!(parsed.value, "untrusted bytes");

    // The Raw token type itself: empty enum, zero-sized.
    let _size: usize = core::mem::size_of::<Raw>();
    assert_eq!(core::mem::size_of::<Raw>(), 0);
}
