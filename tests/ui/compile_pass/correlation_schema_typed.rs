// COMPILE-PASS: CorrelationSchema typed — CorrelatedLog with different SCHEMA values
// are distinct types and each is accepted by its schema-specific function.
//
// Law: CorrelationSchemaLaw — the SCHEMA const str prevents silent schema substitution.
use wasm4pm_compat::correlation::{CorrelatedLog, CorrelationKey, CorrelationWitness};

struct LogA;
struct LogB;

fn requires_by_case_log(_log: CorrelatedLog<LogA, LogB, "by-case">) {}
fn requires_by_object_log(_log: CorrelatedLog<LogA, LogB, "by-object">) {}

fn main() {
    let by_case: CorrelatedLog<LogA, LogB, "by-case"> = CorrelatedLog::new();
    let by_object: CorrelatedLog<LogA, LogB, "by-object"> = CorrelatedLog::new();

    requires_by_case_log(by_case);
    requires_by_object_log(by_object);

    // CorrelationKey schema accessor
    let key: CorrelationKey<"by-timestamp"> = CorrelationKey::new();
    assert_eq!(key.schema(), "by-timestamp");

    // CorrelationWitness is a zero-sized marker
    let _w: CorrelationWitness<"by-attribute"> = CorrelationWitness;
}
