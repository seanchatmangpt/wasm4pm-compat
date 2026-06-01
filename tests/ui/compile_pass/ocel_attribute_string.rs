// Law: OcelAttributeValueStringLaw — OcelAttributeValue::String is a first-class OCEL 2.0 attribute variant; string-valued attributes are constructible and accessible
// COMPILE-PASS: OcelAttributeValue::String — string-valued OCEL attributes are lawfully constructed.
use wasm4pm_compat::ocel::{OcelAttribute, OcelAttributeValue};

fn main() {
    let attr = OcelAttribute::string("status", "open");
    assert_eq!(attr.key, "status");
    assert_eq!(attr.value, OcelAttributeValue::String("open".into()));
}
