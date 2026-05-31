// COMPILE-PASS: TypedObject tag distinctness — OrderTag and ItemTag are non-interchangeable phantom types.
use wasm4pm_compat::ocel::{TypedObject, ObjectTypeTag};

#[derive(Clone, Debug, PartialEq)]
struct OrderTag;
impl ObjectTypeTag for OrderTag {
    const TYPE_NAME: &'static str = "order";
}

#[derive(Clone, Debug, PartialEq)]
struct ItemTag;
impl ObjectTypeTag for ItemTag {
    const TYPE_NAME: &'static str = "item";
}

fn accept_order(_: &TypedObject<OrderTag>) {}
fn accept_item(_: &TypedObject<ItemTag>) {}

fn main() {
    let order = TypedObject::<OrderTag>::new("ord-1");
    let item = TypedObject::<ItemTag>::new("item-3");

    // Each function accepts only its specific tag — type system prevents mixing.
    accept_order(&order);
    accept_item(&item);

    assert_eq!(order.inner().object_type(), "order");
    assert_eq!(item.inner().object_type(), "item");
}
