// COMPILE-PASS: OcelEvent builder — with_attribute and at_ns attach attributes and timestamps to events.
use wasm4pm_compat::ocel::{OcelEvent, OcelAttribute, OcelAttributeValue};

fn main() {
    let event = OcelEvent::new("e1", "ship")
        .at_ns(1_700_000_000_000_000_000)
        .with_attribute(OcelAttribute::string("channel", "web"))
        .with_attribute(OcelAttribute::boolean("express", true));

    assert_eq!(event.id(), "e1");
    assert_eq!(event.activity(), "ship");
    assert_eq!(event.timestamp_ns(), Some(1_700_000_000_000_000_000));
    assert_eq!(event.attributes().len(), 2);
    assert_eq!(event.attributes()[0].value, OcelAttributeValue::String("web".into()));
    assert_eq!(event.attributes()[1].value, OcelAttributeValue::Boolean(true));
}
