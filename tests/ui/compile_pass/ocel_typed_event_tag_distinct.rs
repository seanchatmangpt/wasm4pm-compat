// COMPILE-PASS: TypedEvent tag distinctness — PlaceOrderTag and ShipTag are non-interchangeable phantom types.
use wasm4pm_compat::ocel::{TypedEvent, EventTypeTag};

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

fn accept_place(_: &TypedEvent<PlaceOrderTag>) {}
fn accept_ship(_: &TypedEvent<ShipTag>) {}

fn main() {
    let place = TypedEvent::<PlaceOrderTag>::new("e1");
    let ship = TypedEvent::<ShipTag>::new("e2");

    // Each function accepts only its specific tag.
    accept_place(&place);
    accept_ship(&ship);

    assert_eq!(place.inner().activity(), "place_order");
    assert_eq!(ship.inner().activity(), "ship");
}
