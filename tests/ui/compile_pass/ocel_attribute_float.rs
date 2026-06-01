// Law: OcelAttributeValueFloatLaw — OcelAttributeValue::Float is a first-class OCEL 2.0 attribute variant; float-valued attributes are constructible and accessible
// COMPILE-PASS: OcelAttributeValue::Float — float-valued OCEL attributes are lawfully constructed.
use wasm4pm_compat::ocel::{OcelAttribute, OcelAttributeValue};

fn main() {
    let attr = OcelAttribute {
        key: "price".into(),
        value: OcelAttributeValue::Float(9.99),
    };
    assert_eq!(attr.key, "price");
    assert!(matches!(attr.value, OcelAttributeValue::Float(_)));
    if let OcelAttributeValue::Float(v) = attr.value {
        assert!((v - 9.99).abs() < f64::EPSILON);
    }
}
