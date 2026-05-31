// COMPILE-PASS: Copy semantics law — K need not be Clone or Copy.
//
// Law: All typed id newtypes implement Copy and Clone manually, so K does not
// need to be Clone/Copy/Sized beyond the default. An id can be passed to a
// function and still used after the call, because Copy means no move occurs.
use wasm4pm_compat::ids::{CaseId, EventId, ObjectId, TraceId};

// A kind marker that does not derive Clone or Copy.
enum NoCloneLog {}

fn consume_event(ev: EventId<NoCloneLog>) -> u64 { ev.raw() }
fn consume_object(obj: ObjectId<NoCloneLog>) -> u64 { obj.raw() }

fn main() {
    let ev = EventId::<NoCloneLog>::new(42u64);
    // Pass to function — Copy means the original is still live.
    let raw1 = consume_event(ev);
    let raw2 = consume_event(ev); // still usable
    assert_eq!(raw1, 42u64);
    assert_eq!(raw2, 42u64);

    // Same for ObjectId.
    let obj = ObjectId::<NoCloneLog>::new(7u64);
    assert_eq!(consume_object(obj), 7u64);
    assert_eq!(consume_object(obj), 7u64); // copy — original live

    // Clone explicitly also works.
    let trace = TraceId::<NoCloneLog>::new(3u64);
    let trace_clone = trace.clone();
    assert_eq!(trace, trace_clone);

    // CaseId Copy.
    let case = CaseId::<NoCloneLog>::new(99u64);
    let case2 = case;
    assert_eq!(case, case2); // both live
}
