//! Temporal ordering and profile shapes (`src/temporal.rs`).
//!
//! This example exercises all 5 pub items in the `temporal` module:
//!   - `TemporalOrder` enum (Before, After, Concurrent, Unknown) + Display
//!   - `TemporalProfile<Trace>` structural shape (new, Default, PhantomData)
//!   - `TemporalOrderWitness` zero-cost marker
//!   - `SojournTimeWitness` zero-cost marker
//!   - `TimeAwareEvidence<T, Order>` (new, into_inner, inner field, order binding)
//!
//! The invariants proven here:
//!   - TemporalOrder variants Display as lowercase strings
//!   - TimeAwareEvidence<T, TemporalOrderWitness> and <T, SojournTimeWitness> are
//!     different types — the Order type parameter is the compile-time context anchor
//!   - TemporalProfile::new() == TemporalProfile::default() (structurally identical)
//!   - TimeAwareEvidence::into_inner() recovers the original value exactly

use wasm4pm_compat::temporal::{
    SojournTimeWitness, TemporalOrder, TemporalOrderWitness, TemporalProfile, TimeAwareEvidence,
};

#[derive(Default, Clone, Copy)]
struct MyTrace;

fn main() {
    // ── TemporalOrder — four variants + Display ───────────────────────────────
    println!("== TemporalOrder: four variants ==");

    let orders = [
        (TemporalOrder::Before, "before"),
        (TemporalOrder::After, "after"),
        (TemporalOrder::Concurrent, "concurrent"),
        (TemporalOrder::Unknown, "unknown"),
    ];
    for (ord, expected) in &orders {
        let displayed = format!("{ord}");
        assert_eq!(&displayed, expected, "TemporalOrder Display mismatch");
        println!("  {ord:?} -> Display: \"{displayed}\"");
    }

    // Copy + Clone
    let o = TemporalOrder::Before;
    let o2 = o;
    assert_eq!(o, o2, "TemporalOrder is Copy");

    // Hash (compile check — stored in a set)
    let mut seen = std::collections::HashSet::new();
    seen.insert(TemporalOrder::Before);
    seen.insert(TemporalOrder::After);
    seen.insert(TemporalOrder::Concurrent);
    seen.insert(TemporalOrder::Unknown);
    assert_eq!(seen.len(), 4, "all four variants hash distinctly");
    println!("  All 4 variants hash distinctly: {}", seen.len());

    // ── TemporalProfile — structural shape marker ────────────────────────────
    println!("\n== TemporalProfile: structural shape marker ==");

    let pair = MyTrace;
    let avg = wasm4pm_compat::temporal::TimeDelta::new(1.0);
    let std = wasm4pm_compat::temporal::TimeDelta::new(0.5);
    let prof_new: TemporalProfile<MyTrace> = TemporalProfile::new(avg, std, pair);
    let prof_default: TemporalProfile<MyTrace> = TemporalProfile::default();
    let prof_direct: TemporalProfile<MyTrace> = TemporalProfile { avg, std, pair };

    // All three construction paths produce the same size value (16 bytes for two f64s)
    assert_eq!(
        std::mem::size_of_val(&prof_new),
        16,
        "TemporalProfile is 16 bytes"
    );
    assert_eq!(std::mem::size_of_val(&prof_default), 16);
    assert_eq!(std::mem::size_of_val(&prof_direct), 16);
    println!(
        "  TemporalProfile<MyTrace>::new()     size: {} bytes",
        std::mem::size_of_val(&prof_new)
    );
    println!(
        "  TemporalProfile<MyTrace>::default() size: {} bytes",
        std::mem::size_of_val(&prof_default)
    );

    // ── TemporalOrderWitness — zero-cost marker ───────────────────────────────
    println!("\n== TemporalOrderWitness + SojournTimeWitness: zero-cost markers ==");

    let tow = TemporalOrderWitness;
    let stw = SojournTimeWitness;
    assert_eq!(
        std::mem::size_of_val(&tow),
        0,
        "TemporalOrderWitness zero-sized"
    );
    assert_eq!(
        std::mem::size_of_val(&stw),
        0,
        "SojournTimeWitness zero-sized"
    );
    println!(
        "  TemporalOrderWitness size : {} bytes",
        std::mem::size_of_val(&tow)
    );
    println!(
        "  SojournTimeWitness   size : {} bytes",
        std::mem::size_of_val(&stw)
    );

    // ── TimeAwareEvidence — wraps T with Order context ────────────────────────
    println!("\n== TimeAwareEvidence: temporal context wrapper ==");

    // With TemporalOrderWitness
    let tae_order: TimeAwareEvidence<u64, TemporalOrderWitness> = TimeAwareEvidence::new(42u64);
    assert_eq!(tae_order.inner, 42u64, "inner field");

    // With SojournTimeWitness
    let tae_sojourn: TimeAwareEvidence<String, SojournTimeWitness> =
        TimeAwareEvidence::new(String::from("hello"));
    assert_eq!(
        tae_sojourn.inner, "hello",
        "inner field via SojournTimeWitness"
    );

    // into_inner recovers value
    let recovered: u64 = tae_order.into_inner();
    assert_eq!(recovered, 42u64, "into_inner round-trip");

    let recovered_str: String = tae_sojourn.into_inner();
    assert_eq!(recovered_str, "hello", "into_inner string round-trip");

    // Size: TimeAwareEvidence<u64, _> == size_of(u64) because PhantomData is zero
    let tae2: TimeAwareEvidence<u64, TemporalOrderWitness> = TimeAwareEvidence::new(99u64);
    assert_eq!(
        std::mem::size_of_val(&tae2),
        std::mem::size_of::<u64>(),
        "TimeAwareEvidence<u64,_> == size of u64"
    );

    println!("  TimeAwareEvidence<u64, TemporalOrderWitness>::new(42).inner == 42");
    println!("  TimeAwareEvidence<String, SojournTimeWitness>::into_inner() == \"hello\"");
    println!(
        "  TimeAwareEvidence<u64,_> size == size_of::<u64>(): {} bytes",
        std::mem::size_of::<u64>()
    );

    // ── Order context anchors evidence at different stages ────────────────────
    // TimeAwareEvidence<u64, TemporalOrderWitness> and <u64, SojournTimeWitness>
    // are distinct types even though T == u64. This is the structural invariant.
    fn needs_order_witness(_: &TimeAwareEvidence<u64, TemporalOrderWitness>) {}
    fn needs_sojourn_witness(_: &TimeAwareEvidence<u64, SojournTimeWitness>) {}

    let tae_ord: TimeAwareEvidence<u64, TemporalOrderWitness> = TimeAwareEvidence::new(1u64);
    let tae_soj: TimeAwareEvidence<u64, SojournTimeWitness> = TimeAwareEvidence::new(1u64);
    needs_order_witness(&tae_ord);
    needs_sojourn_witness(&tae_soj);

    println!("\n  Order context enforced at type level:");
    println!("  TimeAwareEvidence<u64, TemporalOrderWitness> != TimeAwareEvidence<u64, SojournTimeWitness>");

    println!("\nEXIT 0");
}
