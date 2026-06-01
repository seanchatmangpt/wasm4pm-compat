// Law: OcelObjectBuilderLaw — OcelObject builder attaches typed attributes via with_attribute; structure-only, no engine logic
// COMPILE-PASS: OcelObject builder — with_attribute attaches typed attributes to objects.
use wasm4pm_compat::ocel::{OcelObject, OcelAttribute, OcelAttributeValue};

fn main() {
    let obj = OcelObject::new("ord-1", "order")
        .with_attribute(OcelAttribute::string("status", "open"))
        .with_attribute(OcelAttribute::float("total", 149.99))
        .with_attribute(OcelAttribute::integer("item_count", 3));

    assert_eq!(obj.id(), "ord-1");
    assert_eq!(obj.object_type(), "order");
    assert_eq!(obj.attributes().len(), 3);
    assert_eq!(obj.attributes()[0].value, OcelAttributeValue::String("open".into()));
    assert_eq!(obj.attributes()[1].value, OcelAttributeValue::Float(149.99));
    assert_eq!(obj.attributes()[2].value, OcelAttributeValue::Integer(3));
}
