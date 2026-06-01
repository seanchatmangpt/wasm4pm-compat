// Law: OcelAttributeValueIntegerLaw — OcelAttributeValue::Integer is a first-class OCEL 2.0 attribute variant; integer-valued attributes are constructible and accessible
// COMPILE-PASS: OcelAttributeValue::Integer — integer-valued OCEL attributes are lawfully constructed.
use wasm4pm_compat::ocel::{OcelAttribute, OcelAttributeValue};

fn main() {
    let attr = OcelAttribute::integer("quantity", 42);
    assert_eq!(attr.key, "quantity");
    assert_eq!(attr.value, OcelAttributeValue::Integer(42));
    if let OcelAttributeValue::Integer(v) = attr.value {
        assert_eq!(v, 42i64);
    }
}
