// COMPILE-PASS: ObjectTypeName dedicated surface — string-backed object-type label.
//
// Law: ObjectTypeName<K> carries the human-readable label of an OCEL object-type
// class (e.g. "order", "item", "payment"). It is structurally distinct from
// EventTypeName<K> even for the same K and the same label string. Confusing
// them is a compile error. Cow<'static, str> enables zero-copy from statics
// and heap allocation from owned Strings.
use wasm4pm_compat::ids::ObjectTypeName;

enum MyLog {}

fn main() {
    // Static label.
    let order = ObjectTypeName::<MyLog>::from_static("order");
    assert_eq!(order.as_str(), "order");

    // Owned label.
    let item = ObjectTypeName::<MyLog>::from_owned(String::from("item"));
    assert_eq!(item.as_str(), "item");

    let payment = ObjectTypeName::<MyLog>::from_static("payment");
    assert_eq!(payment.as_str(), "payment");

    // Equality by label content.
    let order2 = ObjectTypeName::<MyLog>::from_static("order");
    assert_eq!(order, order2);

    // Inequality when labels differ.
    assert_ne!(order, item);

    // Ordering is lexicographic on the label.
    assert!(item < order); // "item" < "order"
    assert!(order < payment); // "order" < "payment"

    // Display format.
    assert_eq!(format!("{}", order), "ObjectTypeName(\"order\")");
    assert_eq!(format!("{}", item), "ObjectTypeName(\"item\")");

    // Clone without K: Clone.
    let order_clone = order.clone();
    assert_eq!(order_clone, order);
}
