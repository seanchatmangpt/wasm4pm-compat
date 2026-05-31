// COMPILE-PASS: OcelAttributeValue::Map — map-valued OCEL attributes are lawfully constructed.
use wasm4pm_compat::ocel::OcelAttributeValue;

fn main() {
    let map = OcelAttributeValue::Map(vec![
        ("currency".to_string(), OcelAttributeValue::String("EUR".into())),
        ("amount".to_string(), OcelAttributeValue::Float(100.0)),
    ]);
    if let OcelAttributeValue::Map(pairs) = &map {
        assert_eq!(pairs.len(), 2);
        assert_eq!(pairs[0].0, "currency");
    }
    let _ = map;
}
