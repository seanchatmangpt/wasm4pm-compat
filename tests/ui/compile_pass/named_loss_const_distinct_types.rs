// Law: NamedLossConstDistinctTypesLaw — NamedLossConst<"DroppedObjectTypeLinks"> and NamedLossConst<"FlattenedMultiObjectRelation"> are distinct types; const-generic names produce non-interchangeable markers
// COMPILE-PASS: NamedLossConst distinct types — proves two different loss category names produce distinct types

#![feature(adt_const_params)]

use wasm4pm_compat::loss::NamedLossConst;

fn accepts_dropped_links(_: NamedLossConst<"DroppedObjectTypeLinks">) {}
fn accepts_flattened_rel(_: NamedLossConst<"FlattenedMultiObjectRelation">) {}

fn main() {
    accepts_dropped_links(NamedLossConst::<"DroppedObjectTypeLinks">);
    accepts_flattened_rel(NamedLossConst::<"FlattenedMultiObjectRelation">);
}
