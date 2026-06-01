// Law: TypedAttributePhantomTagLaw — TypedAttribute<Tag> uses AttributeTypeTag phantom to prevent attribute-domain confusion at compile time; distinct tags are non-interchangeable
// COMPILE-PASS: TypedAttribute — attribute-domain phantom tag prevents domain confusion at compile time.
use wasm4pm_compat::ocel::{OcelAttribute, TypedAttribute, AttributeTypeTag};

#[derive(Clone, Debug, PartialEq)]
struct StatusTag;
impl AttributeTypeTag for StatusTag {
    const ATTR_NAME: &'static str = "status";
}

#[derive(Clone, Debug, PartialEq)]
struct PriceTag;
impl AttributeTypeTag for PriceTag {
    const ATTR_NAME: &'static str = "price";
}

fn main() {
    // Wrap string attribute with domain tag.
    let status_attr = OcelAttribute::string("status", "open");
    let typed_status = TypedAttribute::<StatusTag>::wrap(status_attr);
    assert_eq!(typed_status.inner().key, "status");

    // Wrap float attribute with domain tag.
    let price_attr = OcelAttribute::float("price", 9.99);
    let typed_price = TypedAttribute::<PriceTag>::wrap(price_attr);
    assert_eq!(typed_price.inner().key, "price");

    // into_inner recovers the OcelAttribute.
    let recovered = typed_price.into_inner();
    assert_eq!(recovered.key, "price");
}
