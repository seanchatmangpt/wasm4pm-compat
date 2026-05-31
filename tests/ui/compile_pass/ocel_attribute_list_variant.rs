// COMPILE-PASS: OcelAttributeValue::List — list-valued OCEL attributes are lawfully constructed.
use wasm4pm_compat::ocel::OcelAttributeValue;

fn main() {
    let list = OcelAttributeValue::List(vec![
        OcelAttributeValue::String("alpha".into()),
        OcelAttributeValue::Integer(1),
        OcelAttributeValue::Float(3.14),
    ]);
    if let OcelAttributeValue::List(items) = &list {
        assert_eq!(items.len(), 3);
    }
    let _ = list;
}
