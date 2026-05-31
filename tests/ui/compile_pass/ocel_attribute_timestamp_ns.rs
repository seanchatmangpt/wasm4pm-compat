// COMPILE-PASS: OcelAttributeValue::TimestampNs — timestamp attributes store nanoseconds since Unix epoch.
use wasm4pm_compat::ocel::{OcelAttribute, OcelAttributeValue};

fn main() {
    let ts: i64 = 1_700_000_000_000_000_000;
    let attr = OcelAttribute::timestamp_ns("created_at", ts);
    assert_eq!(attr.key, "created_at");
    assert_eq!(attr.value, OcelAttributeValue::TimestampNs(ts));
    if let OcelAttributeValue::TimestampNs(v) = attr.value {
        assert_eq!(v, ts);
    }
}
