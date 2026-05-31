//! Smoke tests for the part-B process-canon shapes (Agent 3b).
//!
//! These construct each shape and assert its basic structural invariants. They
//! are **structure-only** smoke checks — they never run discovery, conformance,
//! prediction, or replay. They merely prove the shapes build and that their
//! bounded newtypes refuse out-of-range input.

use wasm4pm_compat::conformance::{
    ConformanceRefusal, ConformanceVerdict, Deviation, Fitness, Precision, F1,
};
use wasm4pm_compat::declare::{
    Activity, DeclareConstraint, DeclareRefusal, DeclareScope, DeclareTemplate,
};
use wasm4pm_compat::ocpq::{ObjectScope, OcpqQuery, OcpqRefusal, Predicate, PredicateKind};
use wasm4pm_compat::powl::{OrderEdge, Powl, PowlNode, PowlNodeId, PowlNodeKind, PowlRefusal};
use wasm4pm_compat::prediction::{PredictionProblem, PredictionRefusal, PredictionTarget};
use wasm4pm_compat::process_tree::{
    ProcessTree, ProcessTreeNode, ProcessTreeOperator, ProcessTreeRefusal,
};
use wasm4pm_compat::receipt::{Digest, ReceiptRefusal, ReceiptShape, ReplayHint};

#[test]
fn smoke_powl() {
    // Build a tiny partial order: a -> b.
    let mut p = Powl::new();
    p.nodes
        .push(PowlNode::new(PowlNodeId(0), PowlNodeKind::Atom("a".into())));
    p.nodes
        .push(PowlNode::new(PowlNodeId(1), PowlNodeKind::Atom("b".into())));
    p.nodes.push(PowlNode::new(
        PowlNodeId(2),
        PowlNodeKind::PartialOrder(vec![PowlNodeId(0), PowlNodeId(1)]),
    ));
    p.edges.push(OrderEdge::new(PowlNodeId(0), PowlNodeId(1)));
    p.root = Some(PowlNodeId(2));

    assert_eq!(p.node_count(), 3);
    assert_eq!(p.edges[0].from, PowlNodeId(0));
    assert_eq!(p.root, Some(PowlNodeId(2)));

    // Refusal names a specific law.
    let r = PowlRefusal::CyclicPartialOrder;
    assert_eq!(r, PowlRefusal::CyclicPartialOrder);
    assert!(r.to_string().contains("CyclicPartialOrder"));
}

#[test]
fn smoke_tree() {
    let mut t = ProcessTree::new();
    t.nodes.push(ProcessTreeNode::Activity("a".into()));
    t.nodes.push(ProcessTreeNode::Activity("b".into()));
    t.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Sequence,
        children: vec![
            wasm4pm_compat::process_tree::ProcessTreeNodeId(0),
            wasm4pm_compat::process_tree::ProcessTreeNodeId(1),
        ],
    });
    t.root = Some(wasm4pm_compat::process_tree::ProcessTreeNodeId(2));

    assert_eq!(t.node_count(), 3);
    assert_eq!(
        t.root,
        Some(wasm4pm_compat::process_tree::ProcessTreeNodeId(2))
    );

    let r = ProcessTreeRefusal::InvalidArity;
    assert!(r.to_string().contains("InvalidArity"));
}

#[test]
fn smoke_declare() {
    let c = DeclareConstraint::binary(
        DeclareTemplate::Response,
        Activity::new("a"),
        Activity::new("b"),
        DeclareScope::SingleObjectScope("order".into()),
    );
    assert_eq!(c.template, DeclareTemplate::Response);
    assert_eq!(c.template.arity(), 2);
    assert!(c.target.is_some());

    let u = DeclareConstraint::unary(
        DeclareTemplate::Existence,
        Activity::new("a"),
        DeclareScope::MultiObjectScope(vec!["order".into(), "item".into()]),
    );
    assert_eq!(u.template.arity(), 1);
    assert!(u.target.is_none());

    let r = DeclareRefusal::MissingTarget;
    assert!(r.to_string().contains("MissingTarget"));
}

#[test]
fn smoke_ocpq() {
    let mut q = OcpqQuery::new(ObjectScope::new(["order", "item"]));
    q.predicates.push(Predicate::new(PredicateKind::Event(
        "activity = pay".into(),
    )));
    q.predicates
        .push(Predicate::new(PredicateKind::Cardinality {
            min: 1,
            max: 3,
        }));

    assert!(!q.scope.is_empty());
    assert_eq!(q.scope.object_types.len(), 2);
    assert_eq!(q.predicates.len(), 2);

    let r = OcpqRefusal::FlatteningRequired;
    assert!(r.to_string().contains("FlatteningRequired"));
}

#[test]
fn smoke_conformance() {
    // Bounded newtypes refuse out-of-range input.
    assert!(Fitness::new(1.5).is_none());
    assert!(Fitness::new(-0.1).is_none());
    assert!(Fitness::new(1.0).is_some());
    assert!(Precision::new(2.0).is_none());
    assert!(Precision::new(0.5).is_some());
    assert!(F1::new(f64::NAN).is_none());
    assert!(F1::new(0.5).is_some());

    let mut v = ConformanceVerdict::new();
    assert!(!v.is_perfect());
    v.fitness = Fitness::new(1.0);
    assert!(v.is_perfect());

    v.deviations.push(Deviation::new(0, "skip"));
    assert!(!v.is_perfect());

    let r = ConformanceRefusal::FitnessUnavailable;
    assert!(r.to_string().contains("FitnessUnavailable"));
}

#[test]
fn smoke_prediction() {
    let p = PredictionProblem::<()>::new(
        vec!["register".into(), "review".into()],
        PredictionTarget::NextActivity,
    );
    assert_eq!(p.prefix_len(), 2);
    assert_eq!(p.target, PredictionTarget::NextActivity);

    let r = PredictionRefusal::EmptyPrefix;
    assert!(r.to_string().contains("EmptyPrefix"));
}

#[test]
fn smoke_receipt() {
    let r = ReceiptShape::new(
        "discovery-run",
        Digest::new("blake3:abc123"),
        ReplayHint::new("rerun:plan#1"),
    );
    assert!(r.is_well_shaped());
    assert_eq!(r.witness, "discovery-run");

    let empty = ReceiptShape::new("", Digest::new(""), ReplayHint::new(""));
    assert!(!empty.is_well_shaped());

    let refusal = ReceiptRefusal::MissingDigest;
    assert!(refusal.to_string().contains("MissingDigest"));
}
