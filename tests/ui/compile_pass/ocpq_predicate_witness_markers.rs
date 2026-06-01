// COMPILE-PASS: OCPQ predicate witness markers — all seven are constructible.
//
// Law: The seven canonical OCPQ predicate witness markers (EventPredicate,
// ObjectPredicate, RelationPredicate, TemporalPredicate, CardinalityPredicate,
// NestedQuery, Constraint) are zero-sized types that tag predicates at the type
// level. All seven implement IsOcpqPredicate via the sealed trait.
use wasm4pm_compat::ocpq::{
    CardinalityPredicate, Constraint, EventPredicate, IsOcpqPredicate, NestedQuery,
    ObjectPredicate, RelationPredicate, TemporalPredicate,
};

fn accepts_predicate<W: IsOcpqPredicate>(_: W) {}

fn main() {
    accepts_predicate(EventPredicate);
    accepts_predicate(ObjectPredicate);
    accepts_predicate(RelationPredicate);
    accepts_predicate(TemporalPredicate);
    accepts_predicate(CardinalityPredicate);
    accepts_predicate(NestedQuery);
    accepts_predicate(Constraint);

    // All are zero-sized and Copy.
    let _ep = EventPredicate;
    let _op = ObjectPredicate;
    let _rp = RelationPredicate;
    let _tp = TemporalPredicate;
    let _cp = CardinalityPredicate;
    let _nq = NestedQuery;
    let _c = Constraint;
}
