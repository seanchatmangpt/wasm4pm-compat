// COMPILE-FAIL: CorrelationSchema key — different schemas produce incompatible logs.
//
// Law: CorrelationSchemaLaw — CorrelatedLog<A,B,"by-case"> and CorrelatedLog<A,B,"by-object">
// are distinct types. A function requiring one schema cannot accept the other.
// The SCHEMA const str parameter prevents silent schema substitution at the type level.
use wasm4pm_compat::correlation::CorrelatedLog;

struct LogA;
struct LogB;

fn requires_by_object_log(_log: CorrelatedLog<LogA, LogB, "by-object">) {}

fn main() {
    let by_case: CorrelatedLog<LogA, LogB, "by-case"> = CorrelatedLog::new();
    // This must fail: CorrelatedLog<_, _, "by-case"> is not CorrelatedLog<_, _, "by-object">.
    // The SCHEMA const str distinguishes correlation schemas at the type level.
    requires_by_object_log(by_case);
}
