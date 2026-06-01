// Law: NamedLossConstDisplayLaw — NamedLossConst<NAME> implements Display as the category name baked into the type at compile time
// COMPILE-PASS: NamedLossConst Display — proves Display formats as the category name

#![feature(adt_const_params)]

use wasm4pm_compat::loss::NamedLossConst;

fn main() {
    assert_eq!(
        format!("{}", NamedLossConst::<"DroppedObjectTypeLinks">),
        "DroppedObjectTypeLinks"
    );
    assert_eq!(
        NamedLossConst::<"FlattenedMultiObjectRelation">::NAME,
        "FlattenedMultiObjectRelation"
    );
}
