#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features, dead_code)]

use wasm4pm_compat::ocpq::{OcpqQueryBounded, OcpqQuery};

fn main() {
    let query = OcpqQuery::default();
    let bounded = OcpqQueryBounded::<{&["order", "item"]}, {&["create", "ship"]}>::new(query);

    // This should fail to compile because "payment" is not in OBJECT_TYPES schema
    let _ = bounded.with_object_predicate::<"payment">();
}
