// Law: NamedLossConstTypeLaw — NamedLossConst<NAME> bakes the loss category into the type at compile time; the category is not a runtime string but a type-level constant
// COMPILE-PASS: NamedLossConst — proves compile-time const-generic loss category baked into type

#![feature(adt_const_params)]

use wasm4pm_compat::loss::NamedLossConst;

type DroppedLinks = NamedLossConst<"DroppedObjectTypeLinks">;
type FlattenedRel = NamedLossConst<"FlattenedMultiObjectRelation">;

fn main() {
    assert_eq!(DroppedLinks::NAME, "DroppedObjectTypeLinks");
    assert_eq!(FlattenedRel::NAME, "FlattenedMultiObjectRelation");

    // Display formats as the category name.
    assert_eq!(format!("{}", NamedLossConst::<"DroppedObjectTypeLinks">), "DroppedObjectTypeLinks");
}
