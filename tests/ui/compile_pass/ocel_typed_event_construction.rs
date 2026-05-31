// COMPILE-PASS: TypedEvent typed construction — EventTypeTag phantom enforces activity-type distinction at compile time.
use wasm4pm_compat::ocel::{OcelEvent, TypedEvent, EventTypeTag};

#[derive(Clone, Debug, PartialEq)]
struct PlaceOrderTag;
impl EventTypeTag for PlaceOrderTag {
    const ACTIVITY_NAME: &'static str = "place_order";
}

#[derive(Clone, Debug, PartialEq)]
struct ShipTag;
impl EventTypeTag for ShipTag {
    const ACTIVITY_NAME: &'static str = "ship";
}

fn main() {
    // Direct construction using tag's ACTIVITY_NAME.
    let place = TypedEvent::<PlaceOrderTag>::new("e1");
    assert_eq!(place.inner().id(), "e1");
    assert_eq!(place.inner().activity(), "place_order");

    // Wrap existing OcelEvent.
    let raw = OcelEvent::new("e2", "ship");
    let ship = TypedEvent::<ShipTag>::wrap(raw);
    assert_eq!(ship.inner().activity(), "ship");

    // into_inner recovers the OcelEvent.
    let unwrapped = ship.into_inner();
    assert_eq!(unwrapped.activity(), "ship");
}
