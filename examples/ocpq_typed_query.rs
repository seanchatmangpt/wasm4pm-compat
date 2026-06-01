//! Example: OCPQ typed query construction
//!
//! Demonstrates how Object-Centric Process Query (OCPQ) shapes are built using
//! const-generic type encoding. Scope strategies, predicate families, and
//! cardinality bounds are distinct types at compile time — wrong combinations
//! are rejected by the compiler before any log is ever touched.
//!
//! This example constructs and inspects query shapes only. No query planning,
//! evaluation, or OCEL log access occurs here; those graduate to `wasm4pm`.
//!
//! Run: cargo run --example ocpq_typed_query

#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features, dead_code, unused_imports, unused_features)]

use wasm4pm_compat::ocpq::{
    CardinalityBoundConst, ChildSetBoundConst, EventPredicateKind, ObjectScope, ObjectScopeConst,
    OcpqQuery, OcpqQueryConst, OcpqRefusal, OcpqScopeKind, Predicate, PredicateKind,
    RelationPredicateKind, TypedEventPredicate, TypedRelationPredicate,
};

fn main() {
    scope_kind_encoding();
    typed_event_predicate();
    typed_relation_predicate();
    cardinality_bound_const();
    child_set_bound_const();
    query_composition();
    refusal_surface();
}

// ── 1. Scope kind is encoded in the type, not at runtime ────────────────────

fn scope_kind_encoding() {
    // OcpqScopeKind::Closed and ::Open are different types — a function that
    // requires Closed cannot silently accept Open.
    let closed: ObjectScopeConst<{ OcpqScopeKind::Closed }> =
        ObjectScopeConst::new(["order", "item"]);
    let open: ObjectScopeConst<{ OcpqScopeKind::Open }> = ObjectScopeConst::new([] as [&str; 0]);
    let single: ObjectScopeConst<{ OcpqScopeKind::SingleType }> = ObjectScopeConst::new(["order"]);

    assert_eq!(closed.kind(), OcpqScopeKind::Closed);
    assert_eq!(
        closed.object_types(),
        &["order".to_string(), "item".to_string()]
    );

    assert_eq!(open.kind(), OcpqScopeKind::Open);
    assert!(open.is_empty());

    assert_eq!(single.kind(), OcpqScopeKind::SingleType);
    assert_eq!(single.object_types(), &["order".to_string()]);

    println!(
        "[scope_kind_encoding] closed={:?}  open={:?}  single={:?}",
        closed.kind(),
        open.kind(),
        single.kind()
    );
}

// ── 2. Typed event predicate — sub-kind in the type parameter ───────────────

fn typed_event_predicate() {
    // TypedEventPredicate<{ActivityEquals}> is a different type from
    // TypedEventPredicate<{AttributeEquals}> — the wrong sub-kind at a function
    // boundary is a compile error.
    let activity = TypedEventPredicate::<{ EventPredicateKind::ActivityEquals }>::new("pay");
    let attribute = TypedEventPredicate::<{ EventPredicateKind::AttributeEquals }>::new("cost > 0");
    let time_range =
        TypedEventPredicate::<{ EventPredicateKind::TimestampInRange }>::new("[0, 3600000]");

    assert_eq!(activity.kind(), EventPredicateKind::ActivityEquals);
    assert_eq!(activity.expression(), "pay");

    assert_eq!(attribute.kind(), EventPredicateKind::AttributeEquals);
    assert_eq!(time_range.kind(), EventPredicateKind::TimestampInRange);

    println!(
        "[typed_event_predicate] activity={:?}  attribute={:?}  time_range={:?}",
        activity.kind(),
        attribute.kind(),
        time_range.kind()
    );
}

// ── 3. Typed relation predicate — E2O / O2O / TBE are distinct types ────────

fn typed_relation_predicate() {
    // E2O: event variable relates to object variable (with optional qualifier)
    let e2o = TypedRelationPredicate::<{ RelationPredicateKind::E2O }>::new("e1 → o1 [order]");
    // O2O: two object variables relate (with optional qualifier)
    let o2o = TypedRelationPredicate::<{ RelationPredicateKind::O2O }>::new("o1 → o2");
    // TBE: duration constraint between two event timestamps
    let tbe = TypedRelationPredicate::<{ RelationPredicateKind::TimeBetweenEvents }>::new(
        "TBE(e1,e2,0,3600000)",
    );

    assert_eq!(e2o.kind(), RelationPredicateKind::E2O);
    assert_eq!(o2o.kind(), RelationPredicateKind::O2O);
    assert_eq!(tbe.kind(), RelationPredicateKind::TimeBetweenEvents);

    // The same link types appear in the runtime PredicateKind variants.
    // TypedRelationPredicate carries the kind in the type; PredicateKind carries
    // it in the enum.  Both name the same structural law.
    // Predicate<()> is the unwitnessed form used when the predicate is stored in
    // a heterogeneous collection (OcpqQuery::predicates).
    let e2o_pred: Predicate<()> = Predicate::new(PredicateKind::E2ORelation {
        event_var: "e1".into(),
        object_var: "o1".into(),
        qualifier: Some("order".into()),
    });
    assert!(matches!(e2o_pred.kind, PredicateKind::E2ORelation { .. }));

    println!(
        "[typed_relation_predicate] e2o={:?}  o2o={:?}  tbe={:?}",
        e2o.kind(),
        o2o.kind(),
        tbe.kind()
    );
}

// ── 4. CardinalityBoundConst — MIN <= MAX enforced at compile time ───────────

fn cardinality_bound_const() {
    // [1, 5]: lawful — compiles.
    let bound: CardinalityBoundConst<1, 5> = CardinalityBoundConst::new();
    assert_eq!(bound.min(), 1);
    assert_eq!(bound.max(), 5);

    // [0, 0]: edge case — also lawful.
    let zero: CardinalityBoundConst<0, 0> = CardinalityBoundConst::new();
    assert_eq!(zero.min(), 0);
    assert_eq!(zero.max(), 0);

    // CardinalityBoundConst<5, 2> would NOT compile.
    // The law MIN <= MAX is expressed as a where-bound on the type, so the
    // violation is detected by rustc before this example ever runs.

    println!(
        "[cardinality_bound_const] [1,5] min={} max={}  [0,0] min={} max={}",
        bound.min(),
        bound.max(),
        zero.min(),
        zero.max()
    );
}

// ── 5. ChildSetBoundConst — labelled CBS predicate, also compile-time safe ───

fn child_set_bound_const() {
    // "items" branch must have between 1 and 5 children.
    let items_bound: ChildSetBoundConst<"items", 1, 5> = ChildSetBoundConst::new();
    // "lines" branch must have between 2 and 10 children.
    let lines_bound: ChildSetBoundConst<"lines", 2, 10> = ChildSetBoundConst::new();

    assert_eq!(items_bound.branch_label(), "items");
    assert_eq!(items_bound.min(), 1);
    assert_eq!(items_bound.max(), 5);

    assert_eq!(lines_bound.branch_label(), "lines");
    assert_eq!(lines_bound.min(), 2);
    assert_eq!(lines_bound.max(), 10);

    // ChildSetBoundConst<"items", 5, 2> and ChildSetBoundConst<"items", 1, 5>
    // are rejected at compile time (MIN > MAX).
    // ChildSetBoundConst<"items", 1, 5> and ChildSetBoundConst<"lines", 1, 5>
    // are different types — the branch label is part of the type.

    println!(
        "[child_set_bound_const] items=[{},{}]  lines=[{},{}]",
        items_bound.min(),
        items_bound.max(),
        lines_bound.min(),
        lines_bound.max()
    );
}

// ── 6. Query composition — predicates added to typed query ──────────────────

fn query_composition() {
    // Runtime OcpqQuery with an ObjectScope: straightforward composition.
    let mut query = OcpqQuery::new(ObjectScope::new(["order", "item"]));

    // Add an event predicate: activity equals "approve-order".
    // OcpqQuery::predicates is Vec<Predicate<()>> — push unwitnessed Predicate.
    // The witness family (EventPredicate, RelationPredicate, …) is used when
    // the predicate is constructed in isolation; here the collection is untyped.
    query.predicates.push(Predicate::new(PredicateKind::Event(
        "activity = approve-order".into(),
    )));

    // Add a typed E2O relation predicate: event e1 links to object o1.
    query
        .predicates
        .push(Predicate::new(PredicateKind::E2ORelation {
            event_var: "e1".into(),
            object_var: "o1".into(),
            qualifier: Some("order".into()),
        }));

    // Add a cardinality predicate: between 1 and 10 matches.
    query
        .predicates
        .push(Predicate::new(PredicateKind::Cardinality {
            min: 1,
            max: 10,
        }));

    // Add a CBS predicate: "items" branch has 1..5 children.
    query
        .predicates
        .push(Predicate::new(PredicateKind::ChildSetBound {
            branch_label: "items".into(),
            min: 1,
            max: 5,
        }));

    assert_eq!(query.predicates.len(), 4);
    assert_eq!(query.scope.object_types, vec!["order", "item"]);
    assert!(query.sub_queries.is_empty());

    // Const-generic typed query: scope kind is part of the type signature.
    // OcpqQueryConst::with_predicate also takes Predicate<()> at the collection level.
    let typed_query =
        OcpqQueryConst::<{ OcpqScopeKind::Closed }>::new(ObjectScopeConst::new(["order", "item"]))
            .with_predicate(Predicate::new(PredicateKind::Event(
                "activity = pay".into(),
            )));

    assert_eq!(typed_query.scope_kind(), OcpqScopeKind::Closed);
    assert_eq!(typed_query.predicates().len(), 1);

    println!(
        "[query_composition] runtime predicates={}  typed scope_kind={:?}  typed predicates={}",
        query.predicates.len(),
        typed_query.scope_kind(),
        typed_query.predicates().len()
    );
}

// ── 7. Refusal surface — named laws, never bare InvalidInput ─────────────────

fn refusal_surface() {
    // OcpqRefusal variants are first-class named laws — each variant names the
    // specific structural rule that was violated.  There is no generic
    // "InvalidInput" fallback.
    let laws: &[OcpqRefusal] = &[
        OcpqRefusal::MissingObjectScope,
        OcpqRefusal::UnknownObjectType,
        OcpqRefusal::InvalidCardinality,
        OcpqRefusal::FlatteningRequired,
        OcpqRefusal::InvalidChildSetBound,
        OcpqRefusal::EmptyScopeType,
        OcpqRefusal::ConflictingPredicateKinds,
        OcpqRefusal::UnboundVariable,
    ];

    for law in laws {
        // Every refusal renders a law name via Display.
        let rendered = format!("{law}");
        assert!(rendered.starts_with("OCPQ refused:"));
    }

    println!(
        "[refusal_surface] {} named refusal laws, all render law name",
        laws.len()
    );
}
