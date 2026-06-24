// COMPILE-PASS: temporal ordering shapes — proves the typed shapes for
// temporal event ordering and temporal profiles compile correctly.
//
// Law: The temporal module is structure-only. TemporalOrder, TemporalProfile,
// TemporalOrderWitness, SojournTimeWitness, and TimeAwareEvidence are typed
// shapes. Temporal computation (ordering derivation, sojourn time calculation,
// temporal conformance checking) graduates to wasm4pm.
use core::marker::PhantomData;
use wasm4pm_compat::temporal::{
    SojournTimeWitness, TemporalOrder, TemporalOrderWitness, TemporalProfile, TimeAwareEvidence,
};

struct MyTrace;

fn main() {
    // TemporalOrder — all variants and Display
    let orders = [
        TemporalOrder::Before,
        TemporalOrder::After,
        TemporalOrder::Concurrent,
        TemporalOrder::Unknown,
    ];
    let displays: Vec<_> = orders.iter().map(|o| format!("{}", o)).collect();
    assert_eq!(displays[0], "before");
    assert_eq!(displays[1], "after");
    assert_eq!(displays[2], "concurrent");
    assert_eq!(displays[3], "unknown");

    // TemporalOrder — Clone and Copy semantics
    let o = TemporalOrder::Before;
    let o2 = o;
    assert_eq!(o, o2);

    // TemporalOrder — PartialEq and Eq
    assert_ne!(TemporalOrder::Before, TemporalOrder::After);
    assert_eq!(TemporalOrder::Concurrent, TemporalOrder::Concurrent);

    // TemporalProfile — parameterized by ActivityPair and Unit
    use wasm4pm_compat::temporal::{ActivityPair, Seconds, TimeDelta};
    let pair = ActivityPair::<String, String>::new();
    let profile = TemporalProfile::new(
        TimeDelta::<Seconds>::new(1.5),
        TimeDelta::<Seconds>::new(0.2),
        pair,
    );
    let _ = profile.avg;
    let _ = profile.std;
    let _ = profile.pair;

    // TemporalOrderWitness — zero-cost marker
    let _ordering_witness = TemporalOrderWitness;

    // SojournTimeWitness — zero-cost marker
    let _sojourn_witness = SojournTimeWitness;

    // TimeAwareEvidence — wraps inner value with temporal context
    let tae: TimeAwareEvidence<u64, TemporalOrderWitness> = TimeAwareEvidence::new(42u64);
    assert_eq!(tae.inner, 42);

    // TimeAwareEvidence — PhantomData field is accessible
    let _order: PhantomData<TemporalOrderWitness> = tae.order;

    // TimeAwareEvidence — into_inner unwraps
    let tae2: TimeAwareEvidence<String, SojournTimeWitness> =
        TimeAwareEvidence::new("hello".to_string());
    let inner = tae2.into_inner();
    assert_eq!(inner, "hello");

    // TimeAwareEvidence — different Order types are distinct (TemporalOrderWitness vs SojournTimeWitness)
    let _ta_ordering: TimeAwareEvidence<u32, TemporalOrderWitness> = TimeAwareEvidence::new(1u32);
    let _ta_sojourn: TimeAwareEvidence<u32, SojournTimeWitness> = TimeAwareEvidence::new(2u32);
}
