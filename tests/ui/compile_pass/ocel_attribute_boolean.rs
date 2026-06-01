// Law: OcelAttributeValueBooleanLaw — OcelAttributeValue::Boolean is a first-class OCEL 2.0 attribute variant; boolean-valued attributes are constructible and accessible
// COMPILE-PASS: OcelAttributeValue::Boolean — boolean-valued OCEL attributes are lawfully constructed.
use wasm4pm_compat::ocel::{OcelAttribute, OcelAttributeValue};

fn main() {
    let attr_true = OcelAttribute::boolean("active", true);
    assert_eq!(attr_true.key, "active");
    assert_eq!(attr_true.value, OcelAttributeValue::Boolean(true));

    let attr_false = OcelAttribute::boolean("active", false);
    assert_eq!(attr_false.value, OcelAttributeValue::Boolean(false));

    if let OcelAttributeValue::Boolean(v) = attr_true.value {
        assert!(v);
    }
}
