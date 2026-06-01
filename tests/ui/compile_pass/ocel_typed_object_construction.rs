// Law: TypedObjectConstructionLaw — TypedObject<Tag> uses ObjectTypeTag phantom to enforce object-type distinction at compile time; type is non-forgeable
// COMPILE-PASS: TypedObject typed construction — ObjectTypeTag phantom enforces type distinction at compile time.
use wasm4pm_compat::ocel::{OcelObject, TypedObject, ObjectTypeTag};

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

fn main() {
    // Direct construction using tag's TYPE_NAME.
    let order = TypedObject::<OrderTag>::new("ord-1");
    assert_eq!(order.inner().id(), "ord-1");
    assert_eq!(order.inner().object_type(), "order");

    // Wrap existing OcelObject.
    let raw = OcelObject::new("item-5", "item");
    let item = TypedObject::<ItemTag>::wrap(raw);
    assert_eq!(item.inner().object_type(), "item");

    // into_inner recovers the OcelObject.
    let unwrapped = order.into_inner();
    assert_eq!(unwrapped.id(), "ord-1");
}
