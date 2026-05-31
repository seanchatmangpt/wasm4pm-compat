// COMPILE-PASS: EventId — kind-typed atom-of-process-evidence identifier.
//
// Law: EventId<K> is a #[repr(transparent)] newtype over u64, parameterised by
// a kind marker K. It identifies a single event within a log. It cannot
// substitute for any other id type even when the raw value is identical.
use wasm4pm_compat::ids::EventId;

enum MyLog {}

fn main() {
    let ev = EventId::<MyLog>::new(42u64);
    assert_eq!(ev.raw(), 42u64);

    // Zero sentinel is detectable without knowing the raw type.
    let zero_ev = EventId::<MyLog>::new(0u64);
    use wasm4pm_compat::ids::TypedId;
    assert!(zero_ev.is_zero());
    assert!(!ev.is_zero());

    // Copy: can be passed and still used.
    fn consume(_: EventId<MyLog>) {}
    let ev2 = ev;
    consume(ev2);
    let _ = ev; // copy — original still live
}
